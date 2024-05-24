@group(0) @binding(0) var<storage, read_write> bbh: array<Node>;
@group(0) @binding(1) var<storage, read> vertex_buffer: array<Vertex>;
@group(0) @binding(2) var<storage, read> index_buffer: array<u32>;

struct Vertex {
  position: Vec4,
  normal: Vec4, 
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  left_child: u32,
}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) num_workgroups: vec3<u32>
  ) {

}



