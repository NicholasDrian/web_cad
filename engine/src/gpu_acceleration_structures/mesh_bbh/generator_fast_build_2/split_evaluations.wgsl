@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read> triangle_info: array<TriangleInfo>;
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


struct TriangleInfo {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  l: u32,
  r: u32,
  left_child: u32,
}

fn pcg(v: u32) -> u32
{
  let s: u32 = v * 747796405u + 2891336453u;
	let w: u32 = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
	return (w >> 22u) ^ w;
}

fn add_bbs(a: TriangleInfo, b: TriangleInfo) -> TriangleInfo {
  let min_corner = min(a.min_corner, b.min_corner);
  let max_corner = max(a.max_corner, b.max_corner);
  return TriangleInfo(
    min_corner,
    max_corner,
  );
}


@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>,
) {

  let node_idx = id.x + id.y * size.x;
  let eval_idx = id.z;
  

  let node = tree[node_idx + params.offset];
  let span = node.r - node.l;

  if (span <= params.max_tris_per_leaf) {
    // is leaf, no need to split 
    return;
  }

  let random_u32 = pcg(seed(node_idx, eval_idx));
  let candidate = triangle_info[index_buffer[random_u32 % span + node.l]];
  let candidate_center = (candidate.min_corner + candidate.max_corner) / 2.0;
  var quality = vec3<f32>(0.0, 0.0, 0.0);
  var accumulated_bb = triangle_info[index_buffer[node.l]];

  for (var i = node.l; i < node.r; i++) {
    let info = triangle_info[index_buffer[i]];
    let center = (info.min_corner + info.max_corner) / 2.0;

    // fancy sign() used to eliminate condition
    // double sign used to create -1 or 1
    // rather than -1, 0 or 1
    // s is 1 for left and -1 for right
    let s = sign(sign(candidate_center - center) + vec3<f32>(0.5, 0.5, 0.5));
    quality += s;

    accumulated_bb = add_bbs(accumulated_bb, info);
  }

  var bb_size = accumulated_bb.max_corner - accumulated_bb.min_corner;
  let size_sum = bb_size.x + bb_size.y + bb_size.z;
  bb_size /= size_sum;

  // make quality from 0 to 1
  // 0 bad, 1 good
  quality = (vec3<f32>(f32(span), f32(span), f32(span)) - abs(quality)) / f32(span);


  split_evaluations[node_idx * size.z + eval_idx] = SplitEval (
      candidate_center,
      quality + bb_size * 0.5,
  );


}
