

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> weightedControls: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> basis_funcs_u: array<f32>;
@group(0) @binding(3) var<storage, read> basis_funcs_v: array<f32>;
@group(0) @binding(4) var<storage, read_write> samples: array<vec4<f32>>;

struct Params {
  control_count_u: u32,
  degree_u: u32,
  control_count_v: u32,
  degree_v: u32,
};

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>
  ) {

  var res: vec4<f32> = vec4<f32>(0, 0, 0, 0);
  for (var i: u32 = 0; i <= params.degree; i++) {
    res += weightedControls[s - params.degree + i] * basisFuncs[i + offset];
  }

  samples[id.x] = res;

}
