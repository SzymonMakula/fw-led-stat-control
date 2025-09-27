use crate::matrix::Matrix;
use crate::picture::Picture;
use crate::wasm_module::WasmModule;

pub struct Plugin {
    pub name: String,
    pub description: String,
    pub image_width: usize,
    pub image_height: usize,
    pub(crate) drawer: Box<dyn Picture>,
}

impl From<WasmModule> for Plugin {
    fn from(value: WasmModule) -> Self {
        Self {
            image_width: value.metadata.width,
            image_height: value.metadata.height,
            name: value.metadata.name.clone(),
            description: value.metadata.description.clone(),
            drawer: Box::from(value),
        }
    }
}

impl Picture for Plugin {
    fn draw(&mut self) -> Matrix {
        self.drawer.draw()
    }
}
