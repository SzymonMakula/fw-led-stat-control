use crate::canvas::Canvas;

struct Controller {
    canvas: Canvas,
}

impl Controller {
    fn init() -> Self {
        Self {
            canvas: Canvas::new(),
        }
    }
}
