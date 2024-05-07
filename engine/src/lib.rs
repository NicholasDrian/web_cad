use wasm_bindgen::prelude::*;
use web_sys::console;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen(start)]
pub fn init() {
    env_logger::init();
}

#[wasm_bindgen]
pub fn hello_world() {
    console::log_1(&"hello from rust".into());
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
