pub const TINY_FLOAT: f32 = 0.000001;

pub fn almost_equal(a: f32, b: f32, threshold: f32) -> bool {
    (a - b).abs() <= threshold
}
