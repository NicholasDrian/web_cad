// NOTE: assumes first knot is at 0
// TODO: get rid of this assumption
pub fn create_spans(knots: &[f32], degree: u32, sample_count: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(sample_count as usize);
    /*
    fn span(u: f32) -> u32 {
      var l: u32 = params.degree;
      var h: u32 = params.knotCount - params.degree - 2;
      while l < h {
        let m = (l + h) / 2;
        if (u >= knots[m + 1]) {
          l = m + 1;
        } else {
          h = m;
        }
      }
      return l;
    }
    span(u) is index of last knot <= u
        */
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
