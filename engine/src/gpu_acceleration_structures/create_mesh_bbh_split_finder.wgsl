// test a few splits to find the bes one
// currently using the surface area heuristic (SAH)

@group(0) @binding(0) var<unifrom> params: Params;
@group(0) @binding(0) var<storage, read> bb_buffer: array<BoudingBox>;
@group(0) @binding(0) var<storage, read_write> bbh: array<Node>;

struct BoudingBox {
  min_corner: vec3<f32>,
  max_cornder: vec3<f32>,
  center: vec3<f32>,
}

struct Params {
  bbh_offset: u32,
}

struct Vertex {
  position: Vec4,
  normal: Vec4, 
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  left_child: u32,
}

struct Leaf {
 triangles: array<u32, 8>,
 null_ptr: u32
}

@compute @workgroup_size(1,1,1) 
fn find_splits(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {

}


