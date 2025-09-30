use crate::canvas::Canvas;
use crate::config::Config;

struct Controller {
    canvas: Canvas,
}

impl Controller {
    fn init() -> Self {
        let config = Config::init();

        Self {
            canvas: config.into(),
        }
    }
}
