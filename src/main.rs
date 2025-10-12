use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use byteorder::ByteOrder;
use dbus::channel::Sender;
use serialport;

use crate::controller::Controller;
use crate::picture::Picture;

mod canvas;
mod config;
mod controller;
mod led_controller;
mod matrix;
mod picture;
mod plugin;
mod wasm_module;

fn main() {
    env_logger::init();

    let mut controller = Controller::init();

    loop {
        controller.schedule_paint();

        sleep(Duration::from_millis(250))
    }
}
