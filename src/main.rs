use std::fs;
use std::io::Write;

use byteorder::ByteOrder;
use serialport;
use systemstat::Platform;

use crate::picture::Picture;
use crate::plugin::Plugin;
use crate::wasm_module::WasmModule;

mod canvas;
mod matrix;
mod picture;
mod plugin;
mod wasm_module;

fn main() {
    let wasm_module_bytes = fs::read("./plugins/battery/build/debug.wasm").unwrap();
    let wasm_module = WasmModule::new(wasm_module_bytes);
    let mut plugin = Plugin::from(wasm_module);
    let picture = plugin.draw();

    print!("got picture {:?}", picture);
}

// let mut port = serialport::new("/dev/ttyACM0", 115_200)
//     .timeout(Duration::from_secs(1))
//     .open()
//     .expect("Failed to open port");
// println!("connected to port {:?}", port);
//
// let mut command = vec![0x32, 0xAC, 0x06];
// command.append(&mut vec![255u8; 50]);
// port.write_all(&command).unwrap();
//
// let sys = System::new();
//
// let cpu = sys.cpu_load_aggregate().unwrap();
// sleep(Duration::from_secs(1));
// let load = cpu.done().unwrap();
//
// println!("battery: {:?}", load)
