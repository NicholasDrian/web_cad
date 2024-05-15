// NOTE: assumes first knot is at 0
// TODO: get rid of this assumption
pub fn create_spans(knots: &[f32], degree: u32, sample_count: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(sample_count as usize);
    let mut idx = degree as usize;
    for i in 0..sample_count {
        let u: f32 = i as f32 / (sample_count - 1) as f32 * knots.last().unwrap();
        while idx < knots.len() - degree as usize - 2 && knots[idx + 1] <= u {
            idx += 1;
        }
        res.push(idx as u32);
    }
    res
}
