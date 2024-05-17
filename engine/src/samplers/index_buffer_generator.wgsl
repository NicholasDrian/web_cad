// For populating index buffer

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> index_buffer: array<u32>;

struct Params {
  count_u: u32,
}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>
  ) {
  let offset = id.y * (params.count_u - 1) * 6;
  for (var i: u32 = 0; i < params.count_u - 1; i++) {
    let x1y1 = i + params.count_u * id.y;
    let x2y1 = i + params.count_u * id.y + 1;
    let x1y2 = i + params.count_u * id.y + params.count_u;
    let x2y2 = i + params.count_u * id.y + 1 + params.count_u;
    index_buffer[offset + i * 6 + 0] = x1y1;
    index_buffer[offset + i * 6 + 1] = x2y1;
    index_buffer[offset + i * 6 + 2] = x2y2;
    index_buffer[offset + i * 6 + 3] = x1y1;
    index_buffer[offset + i * 6 + 4] = x2y2;
    index_buffer[offset + i * 6 + 5] = x1y2;
  }
}
