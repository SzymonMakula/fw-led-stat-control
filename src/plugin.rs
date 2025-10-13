use serde::{Serialize, Serializer};

use crate::matrix::Matrix;
use crate::picture::Picture;
use crate::wasm_module::WasmModule;

pub struct Plugin {
    pub name: String,
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

impl From<WasmModule> for Plugin {
    fn from(value: WasmModule) -> Self {
        Self {
            name: value.metadata.name.clone(),
            drawer: Box::from(value),
        }
    }
}

impl Picture for Plugin {
    fn draw(&mut self) -> Matrix {
        self.drawer.draw()
    }
}
