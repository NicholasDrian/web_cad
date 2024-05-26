// test a few splits to find the bes one
// currently using the surface area heuristic (SAH)

@group(0) @binding(1) var<storage, read> vertex_buffer: array<Vertex>;
@group(0) @binding(2) var<storage, read_write> index_buffer: array<u32>;
@group(0) @binding(0) var<storage, read_write> bbh: array<Node>;

struct Vertex {
  position: Vec4,
  normal: Vec4, 
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  left_child: u32,
}

/// can i bytecast node to leaf? 

struct Leaf {
 triangles: array<u32, 8>,
 null_ptr: u32
}

@compute @workgroup_size(1,1,1) 
fn split(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) num_workgroups: vec3<u32>
  ) {

}


