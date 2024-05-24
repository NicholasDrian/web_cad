pub mod geometry;
pub mod gpu_acceleration_structures;
pub mod gpu_frustum_tracing;
pub mod gpu_ray_tracing;
pub mod gpu_samplers;
pub mod instance;
pub mod instance_interface;
pub mod math;
pub mod render;
pub mod scene;
pub mod utils;
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

    #[cfg(test)]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
