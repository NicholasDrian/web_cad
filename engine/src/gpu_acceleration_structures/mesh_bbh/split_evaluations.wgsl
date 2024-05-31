@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read> triangle_bbs: array<BoundingBox>;
@group(0) @binding(3) var<storage, read> tree: array<Node>;
@group(0) @binding(4) var<storage, read_write> split_evaluations: array<SplitEval>;

struct Params {
  offset: u32,
  max_tris_per_leaf: u32,
  split_candidates: u32,
}

struct SplitEval {
  point: vec3<f32>,
  // split quality in each axis
  quality: vec3<u32>,
}

// convert 2D seed to 1D
fn seed(x: u32, y: u32) -> u32 {
    return 19u * x + 47u * y + 101u;
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

fn pcg(v: u32) -> u32
{
  let s: u32 = v * 747796405u + 2891336453u;
	let w: u32 = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
	return (w >> 22u) ^ w;
}

@compute @workgroup_size(1,1,1)
fn main() {

  


}
