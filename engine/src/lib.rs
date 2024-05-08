pub mod instance;
pub mod math;
pub mod render;
pub mod scene;

#[cfg(test)]
pub mod tests;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).expect("Could't initialize logger");
}

#[wasm_bindgen]
pub fn hello_world() {
    log::info!("hello from rust");
}
