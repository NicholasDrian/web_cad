
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(0) var<storage, read> vertex_buffer: array<MeshVertex>;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read_write> bb_buffer: array<BoundingBox>;

BoundingBox {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
}

struct Params {
  mesh_bb: BouddingBox
}

struct MeshVertex {
  position: vec4<f32>,
  normal: vec4<f32>,
}

fn calculate_morton_code(point: vec3<f32>) -> u64 {
  // descretize into 20 integer bits

  // interlace the bits

}

@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  
}
