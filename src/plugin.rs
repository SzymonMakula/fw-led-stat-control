use serde::{Serialize, Serializer};

use crate::matrix::Matrix;
use crate::picture::Picture;
use crate::wasm_module::WasmModule;

pub struct Plugin {
    pub name: String,
    pub(crate) img_height: usize,
    pub(crate) img_width: usize,
    pub(crate) drawer: Box<dyn Picture>,
}

impl Serialize for Plugin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl Picture for Plugin {
    fn draw(&mut self) -> Matrix {
        self.drawer.draw()
    }
}

impl Plugin {
    pub fn from_wasm(wasm: WasmModule, name: String) -> Self {
        Self {
            name,
            img_width: wasm.metadata.width,
            img_height: wasm.metadata.height,
            drawer: Box::from(wasm),
        }
    }
}
