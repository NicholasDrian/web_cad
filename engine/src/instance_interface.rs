use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::instance::Instance;

#[wasm_bindgen]
pub async fn create_instance(canvases: Vec<HtmlCanvasElement>) {
    Instance::create(&canvases).await;
}
