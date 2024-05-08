pub mod instance;
pub mod math;
pub mod render;
pub mod scene;
pub mod viewport;

#[cfg(test)]
pub mod tests;

use wasm_bindgen::prelude::*;

use crate::render::renderer::Renderer;

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).expect("Could't initialize logger");
}

#[wasm_bindgen]
pub async fn hello_world() {
    log::info!("hello from rust");
    let renderer = Renderer::new().await;
}
