use std::sync::atomic::compiler_fence;

use log::warn;
use serde::Serialize;

use crate::canvas::{AddPainterError, Canvas, Painter};
use crate::config::Config;
use crate::led_controller::LEDController;
use crate::plugin::Plugin;
use crate::wasm_module::WasmModule;

pub struct Controller {
    canvas: Canvas,
    led_controller: LEDController,
}

impl Controller {
    pub fn init() -> Self {
        let config = Config::init();

        Self {
            canvas: config.into(),
            led_controller: LEDController::init(),
        }
    }

    pub fn schedule_paint(&mut self) {
        let matrix = self.canvas.paint_matrix();
        self.led_controller.draw_matrix(matrix)
    }

    pub fn add_plugin(
        &mut self,
        name: &str,
        pos_x: usize,
        pos_y: usize,
    ) -> Result<(), AddPainterError> {
        let plugin = Plugin::from(WasmModule::from(name));
        let painter = Painter::new(plugin, pos_x, pos_y);
        let result = self.canvas.add_painter(painter);
        match result {
            Ok(_) => {
                // Serialize Canvas to Config and save it to FS
                toml::from_str::<Config>(&toml::to_string(&self.canvas).unwrap())
                    .unwrap()
                    .save_to_system();
                Ok(())
            }
            Err(err) => {
                warn!(target: "Controller", "Error adding plugin: {:?}", err);
                Err(err)
            }
        }
    }
}
