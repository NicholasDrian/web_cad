@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> triangle_bbs: array<BoundingBox>;
@group(0) @binding(2) var<storage, read> split_evaluations: array<SplitEval>;
@group(0) @binding(3) var<storage, read> prefix_sum: array<u32>;
@group(0) @binding(4) var<storage, read_write> index_buffer: array<u32>;
@group(0) @binding(5) var<storage, read_write> tree: array<Node>;

struct Params {
  offset: u32,
  max_tris_per_leaf: u32,
  split_candidates: u32,
}

struct SplitEval {
  point: vec3<f32>,
  // split quality in each axis
  quality: vec3<f32>,
}

struct BoundingBox {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  center: vec3<f32>,
}

struct Node {
  min_corner: vec3<f32>,
  l: u32,
  max_corner: vec3<f32>,
  r: u32,
  center: vec3<f32>,
  left_child: u32,
}

@compute @workgroup_size(1,1,1)
fn build_next_level(

) {

  }
