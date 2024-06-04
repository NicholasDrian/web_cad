pub fn default_knot_vector(control_count: usize, degree: u32) -> Vec<f32> {
    let mut res = vec![0.0; degree as usize + 1];
    for i in 1..control_count - degree as usize {
        res.push(i as f32);
    }
    for _ in 0..=degree {
        res.push(control_count as f32 - degree as f32);
    }
    res
}
