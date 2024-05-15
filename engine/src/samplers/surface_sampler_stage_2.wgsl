

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> weighted_controls: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> basis_funcs_u: array<f32>;
@group(0) @binding(3) var<storage, read> basis_funcs_v: array<f32>;
@group(0) @binding(4) var<storage, read> spans_u: array<u32>;
@group(0) @binding(5) var<storage, read> spans_v: array<u32>;
@group(0) @binding(6) var<storage, read_write> samples: array<vec4<f32>>;

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

    let sample = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (let i: u32 = 0; i <= params.degree_u; i++) {
      for (let j: u32 = 0; j <= params.degree_v; j++) {
        let idx = i + j * size.x;
        sample += this.weighted_controls[idx] * (basis_funcs_v[idx] * basis_funcs_u[idx]);
      }
    }
    return vec3.create(res[0] / res[3], res[1] / res[3], res[2] / res[3]);

  }
