pub mod instance;
pub mod math;
pub mod render;
pub mod scene;

use wasm_bindgen::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen(start)]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).expect("Could't initialize logger");
}

#[wasm_bindgen]
pub fn hello_world() {
    log::info!("hello from rust");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
