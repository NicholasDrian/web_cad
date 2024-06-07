// calculate triangle bbh and morton code

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(0) var<storage, read> vertex_buffer: array<MeshVertex>;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read_write> triangle_info_buffer: array<TriangleInfo>;


struct Params {
  mesh_bb_min_corner: vec3<f32>,
  mesh_bb_max_corner: vec3<f32>,
  mesh_bb_size: vec3<f32>,
}

struct TriangleInfo {
    min_corner: vec3<f32>,
    max_corner: vec3<f32>,
    morton_code: u64,
}

fn calculate_morton_code(point: vec3<f32>) -> u64 {
  // descretize into 20 integer bits
  let max_size = params.

  // interlace the bits

}

@compute @workgroup_size(1,1,1)
fn main() {}
