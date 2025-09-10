use std::io::Write;

use byteorder::ByteOrder;
use serialport;
use systemstat::Platform;

use crate::picture::Picture;
use crate::plugin::Plugin;

mod canvas;
mod matrix;
mod picture;
mod plugin;

fn main() {
    let mut plugin = Plugin::new("./plugins/battery/build/debug.wasm");
    let picture = plugin.draw();

    print!("got picture {:?}", picture);

    // let wasm_module = std::fs::read("./plugins/battery/build/debug.wasm").unwrap();
    // let mut store = Store::default();
    // let module = Module::new(&store, wasm_module).unwrap();
    //
    // let custom_section_metadata = module
    //     .custom_sections("metadata")
    //     .next()
    //     .unwrap()
    //     .into_vec();
    //
    // println!(
    //     "custom section {:?}",
    //     String::from_utf8(custom_section_metadata).unwrap()
    // );
    // let import_object = imports! {
    //     "env" => {
    //         "abort" => Function::new_typed(
    //             &mut store,
    //             |msg: i32, file: i32, line: i32, col: i32| {
    //                 eprintln!("AssemblyScript abort called at {}:{} (msg_ptr={}, file_ptr={})", line, col, msg, file);
    //             }
    //         )
    //     }
    // };
    // let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    // let add: TypedFunction<(i32, i32), WasmPtr<u8>> = instance
    //     .exports
    //     .get_typed_function(&mut store, "add")
    //     .unwrap();
    //
    // let ptr = add.call_sys(&mut store, 1, 2).unwrap();
    // let memory = instance.exports.get_memory("memory").unwrap();
    // let view = memory.view(&store);
    //
    // let deref_ptr = ptr.deref(&view);
    // let length_buff = view
    //     .copy_range_to_vec(deref_ptr.offset() - 4..deref_ptr.offset())
    //     .unwrap();
    // let length = LittleEndian::read_u32(&length_buff);
    //
    // println!("data_ptr={:?}", length);
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
