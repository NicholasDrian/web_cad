
@group(0) @binding(0) var<params> params: Params;
@group(0) @binding(1) var<storage, read_write> triangle_info_buffer_clone: array<TriangleInfo>;

struct Params {
  offset: u32,
}

struct TriangleInfo {
    min_corner: vec3<f32>,
    max_corner: vec3<f32>,
    morton_code: u64,
}

@compute @workgroup_size(1,1,1)
fn main() {

  }
