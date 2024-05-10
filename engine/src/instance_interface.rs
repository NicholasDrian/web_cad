use wasm_bindgen::prelude::*;
use web_sys::{js_sys, HtmlCanvasElement};

use crate::instance::{Instance, InstanceHandle, INSTANCES};

#[wasm_bindgen]
pub async fn create_instance(canvases: Vec<HtmlCanvasElement>) -> InstanceHandle {
    Instance::create(&canvases).await
}

#[wasm_bindgen]
pub fn draw(instance_handle: InstanceHandle) {
    INSTANCES
        .lock()
        .unwrap()
        .get(&instance_handle)
        .unwrap()
        .draw();
}
