use wasm_bindgen::prelude::*;

use crate::instance::Handle;

#[wasm_bindgen]
struct Viewport {
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
}
