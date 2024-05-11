use wasm_bindgen::prelude::*;
use web_sys::{js_sys, HtmlCanvasElement};

use crate::instance::{Handle, Instance, INSTANCES};

#[wasm_bindgen]
pub async fn create_instance() -> Handle {
    Instance::create().await
}

#[wasm_bindgen]
pub fn add_viewport(instance_handle: Handle, canvas: HtmlCanvasElement) -> Handle {
    INSTANCES
        .lock()
        .unwrap()
        .get_mut(&instance_handle)
        .unwrap()
        .add_viewport(canvas)
}

#[wasm_bindgen]
pub fn add_scene(instance_handle: Handle) -> Handle {
    INSTANCES
        .lock()
        .unwrap()
        .get_mut(&instance_handle)
        .unwrap()
        .add_scene()
}

#[wasm_bindgen]
pub fn draw_scene_to_all_viewports(instance_handle: Handle, scene_handle: Handle) {
    INSTANCES
        .lock()
        .unwrap()
        .get(&instance_handle)
        .unwrap()
        .draw_scene_to_all_viewports(scene_handle);
}

#[wasm_bindgen]
pub fn draw_scene_to_viewport(
    instance_handle: Handle,
    scene_handle: Handle,
    viewport_handle: Handle,
) {
    INSTANCES
        .lock()
        .unwrap()
        .get(&instance_handle)
        .unwrap()
        .draw_scene_to_viewport(scene_handle, viewport_handle);
}
