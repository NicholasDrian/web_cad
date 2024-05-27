@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;

struct Params {
  resolution: u32
}

@compute @workgroup_size(1,1,1)
fn iota(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let start = id.x * params.resolution;
  for (var i: u32 = 0; i < params.resolution; i++) {
    output[start + i] = start + i;  
  }
}

