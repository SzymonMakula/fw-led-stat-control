use std::env::current_exe;
use std::fs;
use std::io::{Error, ErrorKind};
use std::iter::Flatten;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant, UNIX_EPOCH};

use battery::{Batteries, Battery, Manager};
use log::{error, warn};
use serde::Deserialize;
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, RefreshKind, System,
};
use wasmer::{Function, imports, Imports, Instance, Module, Store, TypedFunction, WasmPtr};
use wasmer_compiler_singlepass::Singlepass;

use crate::matrix::Matrix;
use crate::picture::Picture;

pub struct WasmModule {
    instance: Instance,
    pub(crate) metadata: Metadata,
    store: Store,
}

#[derive(Debug)]
enum WASMError {
    NoCustomSection,
    InvalidCustomSection,
}

impl From<&str> for WasmModule {
    fn from(value: &str) -> Self {
        let module = current_exe()
            .map(|path| path.as_path().parent().map(PathBuf::from))
            .ok()
            .flatten()
            .map(|path| path.join(format!("plugins/{}.wasm", value)))
            .ok_or(std::io::Error::from(ErrorKind::Other))
            .and_then(fs::canonicalize)
            .and_then(fs::read);

        Self::from(module.unwrap_or_else(|err| {
            match err.kind() {
                ErrorKind::NotFound => {
                    error!(
                        "WASM module with name {} was not found in the plugins directory.",
                        value
                    );
                }
                _ => {
                    error!("Unknown error occurred {}", err);
                }
            }
            std::process::exit(1)
        }))
    }
}

impl From<Vec<u8>> for WasmModule {
    fn from(value: Vec<u8>) -> Self {
        let compiler = Singlepass::default();
        let mut store = Store::new(compiler);
        let module = Module::new(&store, value).unwrap_or_else(|err| {
            error!(target: "WASM","Failed to compile WASM module: {}", err);
            std::process::exit(1)
        });
        let import_object = create_imports(&mut store);

        let instance = Instance::new(&mut store, &module, &import_object).unwrap_or_else(|err| {
            error!(target: "WASM","Failed to construct module Instance: {}", err);
            std::process::exit(1)
        });

        let metadata = module
            .custom_sections("metadata")
            .next()
            .ok_or(WASMError::NoCustomSection)
            .map(Vec::from)
            .map(String::from_utf8)
            .and_then(|val| val.or(Err(WASMError::InvalidCustomSection)))
            .and_then(|str| {
                serde_json::from_str::<Metadata>(&str).or(Err(WASMError::InvalidCustomSection))
            })
            .unwrap_or_else(|err| {
                error!(target: "WASM","Failed to read 'metadata' custom section: {:?}", err);
                std::process::exit(1);
            });

        Self {
            instance,
            store,
            metadata,
        }
    }
}

impl Picture for WasmModule {
    fn draw(&mut self) -> Matrix {
        let draw_function: TypedFunction<(), WasmPtr<u8>> = self
            .instance
            .exports
            .get_typed_function(&mut self.store, "draw")
            .unwrap_or_else(|err| {
                error!(target: "WASM", "Failed to get 'draw' function at '{}' module, with error: '{}' ", self.metadata.name, err);
                std::process::exit(1);
            });

        let picture_ptr = draw_function
            .call_sys(&mut self.store)
            .unwrap_or_else(|err| {
                error!(target: "WASM", "Call to 'draw' function failed at '{}' module, with error: {}", self.metadata.name, err);
                std::process::exit(1);
            });

        let view = self
            .instance
            .exports
            .get_memory("memory").unwrap_or_else(|err| {
            error!(target: "WASM", "Could not retrieve memory at key 'memory' at '{}' module, with error: {}",self.metadata.name, err);
            std::process::exit(1);
        })
            .view(&self.store);

        let picture_deref_ptr = picture_ptr.deref(&view);

        let payload_length = self.metadata.width * self.metadata.height;

        // Picture data has contiguous memory allocation, starting at pointer offset and ending at offset + payload len
        let picture = view
            .copy_range_to_vec(
                picture_deref_ptr.offset()..picture_deref_ptr.offset() + payload_length as u64,
            )
            .unwrap_or_else(|_| {
                error!(target: "WASM", "Failed to copy picture data at '{}' module", self.metadata.name);
                std::process::exit(1)
            });

        // Map picture to a 9x39 matrix
        Matrix::from_picture(picture, self.metadata.width, self.metadata.height)
    }
}

#[derive(Deserialize)]
pub struct Metadata {
    pub height: usize,
    pub width: usize,
    pub(crate) name: String,
}

fn create_imports(store: &mut Store) -> Imports {
    let get_global_cpu_usage = || SYSTEM_STAT_MONITOR.lock().unwrap().get_global_cpu_usage();
    let get_memory_usage = || SYSTEM_STAT_MONITOR.lock().unwrap().get_memory_usage();

    imports! {
        "env" => {
            "abort" => Function::new_typed(store, abort_polyfill),
            "get_battery_state_of_charge" => Function::new_typed(store, SystemStatMonitor::get_battery_state_of_charge),
            "get_global_cpu_usage" => Function::new_typed(store, get_global_cpu_usage),
            "get_memory_usage" => Function::new_typed(store, get_memory_usage),
            "get_epoch_time" => Function::new_typed(store, SystemStatMonitor::get_epoch_time),
        }
    }
}

fn abort_polyfill(msg: i32, file: i32, line: i32, col: i32) {
    warn!(
        "AssemblyScript abort called at {}:{} (msg_ptr={}, file_ptr={})",
        line, col, msg, file
    );
}

struct SystemStatMonitor {
    system_api: System,
    last_refresh: Instant,
}

impl SystemStatMonitor {
    fn new() -> Self {
        Self {
            system_api: System::new(),
            last_refresh: Instant::now(),
        }
    }

    fn get_memory_usage(&mut self) -> f32 {
        self.check_refresh();

        let system = &self.system_api;
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();

        let ratio = used_memory as f32 / total_memory as f32;

        ratio
    }

    fn get_global_cpu_usage(&mut self) -> f32 {
        self.check_refresh();
        self.system_api.global_cpu_usage()
    }

    fn get_battery_state_of_charge() -> f32 {
        let mut batteries = Manager::new()
            .unwrap_or_else(|err| {
                error!(target: "SystemStatMonitor", "Failed to instantiate Battery Manager {}", err);
                std::process::exit(1)
            })
            .batteries()
            .unwrap_or_else(|err| {
                error!(target: "SystemStatMonitor", "Failed to construct Batteries iterator {}", err);
                std::process::exit(1)
            });
        let first_battery = batteries
            .next()
            .unwrap_or_else(|| {
                error!(target: "SystemStatMonitor", "No batteries found in the 'Batteries' iterator");
                std::process::exit(1)
            })
            .unwrap_or_else(|err| {
                error!(target: "SystemStatMonitor", "Failed to get battery information {}", err);
                std::process::exit(1)
            });

        first_battery.state_of_charge().value
    }

    fn get_epoch_time() -> u64 {
        let time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        time
    }

    fn check_refresh(&mut self) {
        if Instant::now().duration_since(self.last_refresh) > MINIMUM_CPU_UPDATE_INTERVAL {
            self.system_api.refresh_specifics(
                RefreshKind::nothing()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            );
            self.last_refresh = Instant::now()
        }
    }
}

static SYSTEM_STAT_MONITOR: LazyLock<Mutex<SystemStatMonitor>> =
    LazyLock::new(|| Mutex::new(SystemStatMonitor::new()));
