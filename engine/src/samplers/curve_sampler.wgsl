
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> weightedControls: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> knots: array<f32>;
@group(0) @binding(3) var<storage, read> spans: array<u32>;
@group(0) @binding(4) var<storage, read_write> basisFuncs: array<f32>;
@group(0) @binding(5) var<storage, read_write> samples: array<vec4<f32>>;

struct Params {
  controlCount: u32,
  knotCount: u32,
  degree: u32,
};


@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>
  ) {

  let u: f32 = f32(id.x) / f32(size.x - 1) * knots[params.knotCount - 1];
  let s = spans[id.x];


  let offset: u32 = id.x * (params.degree + 1);
  basisFuncs[offset] = 1.0;
  for (var j: u32 = 1; j <= params.degree; j++) {
    var saved: f32 = 0;
    for (var r: u32 = 0; r < j; r++) {
      let left: f32 = u - knots[s - (j - r) + 1];
      let right: f32 = knots[s + r + 1] - u;
      let temp: f32 = basisFuncs[r + offset] / (right + left);
      basisFuncs[r + offset] = saved + right * temp;
      saved = left * temp;
    }
    basisFuncs[j + offset] = saved;
  }

  var res: vec4<f32> = vec4<f32>(0, 0, 0, 0);
  for (var i: u32 = 0; i <= params.degree; i++) {
    res += weightedControls[s - params.degree + i] * basisFuncs[i + offset];
  }

  samples[id.x] = res;

}

