use std::thread::sleep;
use std::time::Duration;


use crate::controller::Controller;

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
