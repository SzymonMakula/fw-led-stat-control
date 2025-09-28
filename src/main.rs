use std::io::Write;

use byteorder::ByteOrder;
use serialport;
use systemstat::Platform;

use crate::canvas::Canvas;
use crate::config::Config;
use crate::picture::Picture;

mod canvas;
mod config;
mod controller;
mod matrix;
mod picture;
mod plugin;
mod wasm_module;

fn main() {
    let mut canvas: Canvas = Config::init().into();
    let matrix = canvas.paint_matrix();
    println!("matrix {:?} and {:?}", matrix, canvas.painters.len())
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
