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
  quality: vec3<f32>,
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
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>,
) {

  let node = tree[id.x + params.offset];
  let span = node.r - node.l;

  if (span <= max_tris_per_leaf) {
    // is leaf, no need to split 
    return;
  }

  let random_u32 = pcg(seed(id.x, id.y));

  let candidate_idx = index_buffer[random_u32 % span + node.l];
  let candidate_center = triangle_bbs[candidate_idx].center;
  var quality = vec3<f32>(0.0, 0.0, 0.0);

  for (var i = node.l; i < node.r; i++) {
    let center = triangle_bbs[index_buffer[i]].center;

    // fancy sign() used to eliminate condition
    // double sign used to create -1 or 1
    // rather than -1, 0 or 1
    // s is 1 for left and -1 for right
    let s = sign(sign(candidate_center - center) + vec3<f32>(0.5, 0.5, 0.5));
    quality += s; 
  }

  split_evaluations[id.x * size.y + id.y] = SplitEval (
      candidate_center,
      abs(quality)
  );


}
