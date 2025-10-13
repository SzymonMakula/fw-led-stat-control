use std::fs;
use std::path::Path;
use std::sync::{LazyLock, Mutex};
use std::time::UNIX_EPOCH;

use battery::{Battery, Manager};
use log::warn;
use serde::Deserialize;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use wasmer::{Function, imports, Imports, Instance, Module, Store, TypedFunction, WasmPtr};

use crate::matrix::Matrix;
use crate::picture::Picture;

pub struct WasmModule {
    instance: Instance,
    pub metadata: Metadata,
    store: Store,
}

impl From<&str> for WasmModule {
    fn from(value: &str) -> Self {
        let module = fs::read(Path::new(PLUGINS_DIR).join(&format!("{}.wasm", value))).unwrap();
        Self::from(module)
    }
}

impl From<Vec<u8>> for WasmModule {
    fn from(value: Vec<u8>) -> Self {
        let mut store = Store::default();
        let module = Module::new(&store, value).expect("Failed to construct WASM module");
        let import_object = create_imports(&mut store);

        let instance = Instance::new(&mut store, &module, &import_object)
            .expect("Failed to instantiate WASM module");

        let metadata_buffer =
            String::from_utf8(module.custom_sections("metadata").next().unwrap().to_vec()).unwrap();

        let metadata: Metadata = serde_json::from_str(&metadata_buffer).unwrap();

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
            .unwrap();

        let picture_ptr = draw_function
            .call_sys(&mut self.store)
            .expect("Exported function call failed");

        let view = self
            .instance
            .exports
            .get_memory("memory")
            .expect("Failed to get exported memory")
            .view(&self.store);

        let picture_deref_ptr = picture_ptr.deref(&view);

        let payload_length = self.metadata.width * self.metadata.height;

        // Picture data has contiguous memory allocation, starting at pointer offset and ending at offset + payload len
        let picture = view
            .copy_range_to_vec(
                picture_deref_ptr.offset()..picture_deref_ptr.offset() + payload_length as u64,
            )
            .unwrap();

        // Map picture to a 9x39 matrix
        Matrix::from_picture(picture, self.metadata.width, self.metadata.height)
    }
}

#[derive(Deserialize)]
pub struct Metadata {
    pub name: String,
    pub height: usize,
    pub width: usize,
}

fn create_imports(store: &mut Store) -> Imports {
    imports! {
        "env" => {
            "abort" => Function::new_typed(store, abort_polyfill),
               "seed" => Function::new_typed(store, || {
                0.0f64
            }),
            "get_battery_state_of_charge" => Function::new_typed(store, get_battery_state_of_charge),
            "get_global_cpu_usage" => Function::new_typed(store, get_global_cpu_usage),
            "get_memory_usage" => Function::new_typed(store, get_memory_usage),
            "get_epoch_time" => Function::new_typed(store, get_epoch_time),
        }
    }
}

fn abort_polyfill(msg: i32, file: i32, line: i32, col: i32) {
    warn!(
        "AssemblyScript abort called at {}:{} (msg_ptr={}, file_ptr={})",
        line, col, msg, file
    );
}

fn get_battery_state_of_charge() -> f32 {
    get_battery().state_of_charge().value
}

fn get_global_cpu_usage() -> f32 {
    SYSTEM_LOCK.lock().unwrap().refresh_cpu_usage();
    let usage = SYSTEM_LOCK.lock().unwrap().global_cpu_usage();

    usage
}

fn get_battery() -> Battery {
    Manager::new()
        .unwrap()
        .batteries()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
}

fn get_memory_usage() -> f32 {
    let mut system = SYSTEM_LOCK.lock().unwrap();
    system.refresh_memory();
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();

    let ratio = used_memory as f32 / total_memory as f32;

    ratio
}

fn get_epoch_time() -> u64 {
    let time = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    time
}

const PLUGINS_DIR: &'static str = "./plugins";

static SYSTEM_LOCK: LazyLock<Mutex<System>> = LazyLock::new(|| {
    Mutex::new(System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    ))
});
