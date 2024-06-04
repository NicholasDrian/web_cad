use wasm_bindgen::prelude::*;

// TODO: use a more sophisticated way of allocating samples.
// Could take into account curvature, knot distribution, degree...
pub const SAMPLES_PER_SEGMENT: u32 = 10;

#[wasm_bindgen]
pub fn get_samples_per_segment() -> u32 {
    SAMPLES_PER_SEGMENT
}

#[wasm_bindgen]
pub fn set_samples_per_segment(samples: u32) {
    // NOTE: This will be danderous one SAMPLES_PER_SEGMENT is mutable
    todo!()
}
