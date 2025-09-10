use serde::Serialize;

use crate::config::PluginConf;
use crate::matrix::{EMPTY_MATRIX, Matrix, MATRIX_WIDTH};
use crate::picture::Picture;
use crate::wasm_module::WasmModule;

#[derive(Serialize)]
pub struct Plugin {
    pub(crate) name: String,
    #[serde(skip)]
    pub(crate) img_height: usize,
    #[serde(skip)]
    pub(crate) img_width: usize,
    #[serde(rename = "pos_x")]
    pub(crate) offset_x: usize,
    #[serde(rename = "pos_y")]
    pub(crate) offset_y: usize,
    #[serde(skip)]
    pub(crate) drawer: Box<dyn Picture>,
}

impl Picture for Plugin {
    fn draw(&mut self) -> Matrix {
        self.drawer.draw()
    }
}

impl Plugin {
    pub(crate) fn from_plugin_config(plugin_conf: PluginConf) -> Self {
        let wasm_module = WasmModule::from(plugin_conf.name.as_str());
        Self {
            name: plugin_conf.name,
            img_height: wasm_module.metadata.height,
            img_width: wasm_module.metadata.width,
            offset_y: plugin_conf.pos_y,
            offset_x: plugin_conf.pos_x,
            drawer: Box::from(wasm_module),
        }
    }

    // Returns space taken by Picture as Matrix. non 0 values indicate space taken.
    pub(crate) fn get_space_as_matrix(&self) -> Matrix {
        let mut output = Vec::from(EMPTY_MATRIX);

        for row in 0..self.img_height {
            for col in 0..self.img_width {
                output[row * MATRIX_WIDTH + col] = 1
            }
        }
        let matrix = Matrix::try_from(output.as_slice()).expect("Matrix should match dimension");

        matrix.shift_matrix(self.offset_x, self.offset_y)
    }
}
