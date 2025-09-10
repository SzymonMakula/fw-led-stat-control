use byteorder::{ByteOrder, LittleEndian};
use log::warn;
use serde::Deserialize;
use wasmer::{Function, imports, Imports, Instance, Module, Store, TypedFunction, WasmPtr};

use crate::matrix::{EMPTY_MATRIX, Matrix, MATRIX_WIDTH};
use crate::picture::Picture;

pub struct Plugin {
    instance: Instance,
    store: Store,
    pub name: String,
    pub description: String,
    pub image_width: u8,
    pub image_height: u8,
}

#[derive(Deserialize)]
struct Metadata {
    name: String,
    description: String,
    height: u8,
    width: u8,
}

impl Plugin {
    pub fn new(path: &str) -> Self {
        let wasm_module = std::fs::read(path).expect("Failed to resolve plugin at specified path");

        let mut store = Store::default();
        let module = Module::new(&store, wasm_module).expect("Failed to construct WASM module");
        let import_object = create_imports(&mut store);

        let instance = Instance::new(&mut store, &module, &import_object)
            .expect("Failed to instantiate WASM module");

        let metadata_buffer =
            String::from_utf8(module.custom_sections("metadata").next().unwrap().to_vec()).unwrap();

        let metadata: Metadata = serde_json::from_str(&metadata_buffer).unwrap();

        Self {
            instance,
            store,
            name: metadata.name,
            description: metadata.description,
            image_height: metadata.height,
            image_width: metadata.width,
        }
    }
}

impl Picture for Plugin {
    fn draw(&mut self) -> Matrix {
        let draw_function: TypedFunction<(), WasmPtr<u8>> = self
            .instance
            .exports
            .get_typed_function(&mut self.store, "draw")
            .unwrap();

        let picture_ptr = draw_function
            .call_sys(&mut self.store)
            .expect("Exported function call failed");

        let view = self
            .instance
            .exports
            .get_memory("memory")
            .expect("Failed to get exported memory")
            .view(&self.store);

        let picture_deref_ptr = picture_ptr.deref(&view);
        // Payload length is at -4 offset to the initial pointer
        let length_buff = LittleEndian::read_u32(
            &view
                .copy_range_to_vec(
                    picture_deref_ptr.offset() - PAYLOAD_LENGTH_OFFSET..picture_deref_ptr.offset(),
                )
                .unwrap(),
        );

        // Picture data has contiguous memory allocation, starting at pointer offset and ending at offset + payload len
        let picture = view
            .copy_range_to_vec(
                picture_deref_ptr.offset()..picture_deref_ptr.offset() + length_buff as u64,
            )
            .unwrap();

        // Map picture to a 9x39 matrix
        Matrix::from_picture(
            picture,
            self.image_width as usize,
            self.image_height as usize,
        )
    }
}

const PAYLOAD_LENGTH_OFFSET: u64 = 4;

fn create_imports(store: &mut Store) -> Imports {
    imports! {
        "env" => {
            "abort" => Function::new_typed(store, abort_polyfill)
        }
    }
}

fn abort_polyfill(msg: i32, file: i32, line: i32, col: i32) {
    warn!(
        "AssemblyScript abort called at {}:{} (msg_ptr={}, file_ptr={})",
        line, col, msg, file
    );
}
