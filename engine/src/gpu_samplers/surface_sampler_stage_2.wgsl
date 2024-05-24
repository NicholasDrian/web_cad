// compute samples

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

    var sample = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    let span_u = spans_u[id.x];
    let span_v = spans_v[id.y];
    let u_offset = id.x * (params.degree_u + 1);
    let v_offset = id.y * (params.degree_v + 1);

    for (var i: u32 = 0; i <= params.degree_u; i++) {
      for (var j: u32 = 0; j <= params.degree_v; j++) {
        let idx_x = span_u - params.degree_u + i;
        let idx_y = span_v - params.degree_v + j;
        let idx = idx_x + idx_y * params.control_count_u;
        sample += weighted_controls[idx] * (basis_funcs_u[u_offset + i] * basis_funcs_v[v_offset + j]);
      }
    }
    sample /= sample.w;
    samples[(id.x + id.y * size.x) * 2] = sample;
  }
