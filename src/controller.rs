use crate::canvas::Canvas;
use crate::config::Config;
use crate::led_controller::LEDController;

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

    pub fn reload_config(&mut self) {
        let config = Config::init();

        self.canvas = config.into()
    }

    pub fn schedule_paint(&mut self) {
        let matrix = self.canvas.paint_matrix();
        self.led_controller.draw_matrix(matrix)
    }
}

pub enum ControllerMessage {
    ReloadConfig,
    Terminate,
}
