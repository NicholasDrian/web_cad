// In phase1 the basis funcs are evaluated.
// this creates basis funcs evaludated for each u.

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> knots: array<f32>;
@group(0) @binding(2) var<storage, read> spans: array<u32>;
@group(0) @binding(3) var<storage, read_write> basis_funcs: array<f32>;

struct Params {
  control_count: u32,
  knot_count: u32,
  degree: u32,
};


@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>
  ) {

  let u: f32 = f32(id.x) / f32(size.x - 1) * knots[params.knot_count - 1];
  let s: u32 = spans[id.x];
  let offset: u32 = id.x * (params.degree + 1);

  basis_funcs[offset] = 1.0;
  for (var j: u32 = 1; j <= params.degree; j++) {
    var saved: f32 = 0;
    for (var r: u32 = 0; r < j; r++) {
      let left: f32 = u - knots[s - (j - r) + 1];
      let right: f32 = knots[s + r + 1] - u;
      let temp: f32 = basis_funcs[r + offset] / (right + left);
      basis_funcs[r + offset] = saved + right * temp;
      saved = left * temp;
    }
    basis_funcs[j + offset] = saved;
  }
}

