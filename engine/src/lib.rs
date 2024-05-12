pub mod geometry;
pub mod instance;
pub mod instance_interface;
pub mod math;
pub mod render;
pub mod samplers;
pub mod scene;
pub mod viewport;

#[cfg(test)]
pub mod tests;

#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).expect("Could't initialize logger");
}
