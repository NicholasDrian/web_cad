use wasm_bindgen::prelude::*;
use web_sys::{js_sys, HtmlCanvasElement};

use crate::instance::{Instance, InstanceHandle};

#[wasm_bindgen]
pub async fn create_instance(canvases: Vec<HtmlCanvasElement>) -> InstanceHandle {
    Instance::create(&canvases).await
}
