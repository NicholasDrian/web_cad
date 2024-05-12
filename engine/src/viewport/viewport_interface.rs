use wasm_bindgen::prelude::*;

use crate::instance::Handle;

#[wasm_bindgen]
pub struct Viewport {
    instance_handle: Handle,
    viewport_handle: Handle,
}

#[wasm_bindgen]
impl Viewport {
    #[wasm_bindgen(constructor)]
    pub fn new(instance_handle: Handle, viewport_handle: Handle) -> Viewport {
        Viewport {
            instance_handle,
            viewport_handle,
        }
    }
    pub fn get_handle(&self) -> Handle {
        self.viewport_handle
    }
}
