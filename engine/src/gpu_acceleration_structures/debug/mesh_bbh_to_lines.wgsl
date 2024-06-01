@group(0) @binding(0) var<storage, read> tree: array<Node>;
@group(0) @binding(1) var<storage, read_write> vertex_buffer: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> index_buffer: array<u32>;


struct Node {
  min_corner: vec3<f32>,
  l: u32,
  max_corner: vec3<f32>,
  r: u32,
  center: vec3<f32>,
  left_child: u32,
}

@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>
  ) {

  }
