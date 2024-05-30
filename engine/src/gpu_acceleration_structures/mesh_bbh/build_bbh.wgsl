// buid bbh and cull leaf segments
// will do this linearly for now
// need to return updated segment count to gpu

@group(0) @binding(0) var<uniform> params: Parmas;

struct Params {
  tree_size: u32,
}

// 
// if triangle_count > 0, is leaf
// if triangle_count == 0 left_child is actual left_child,
// if triangle_count == -1 invalid node
struct Node {
  min_corner: vec3<f32>,
  triangle_count: i32,
  max_corner: vec3<f32>,
  left_child: u32,
}

@compute @workgroup_size(1,1,1)
fn main() {

}

