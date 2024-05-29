// test a few splits to find the bes one
// currently using the surface area heuristic (SAH)

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> triangle_info_buffer: array<BoudingBox>;
@group(0) @binding(2) var<storage, read> segments: array<Split>;
@group(0) @binding(3) var<storage, read_write> split_candidates: array<vec3<f32>>;


struct Params {
  // number of evaluated splits per input segment.
  // Slows creation, but speeds raytracing.
  candidates_per_segment: u32,
  triangle_count: u32,
  }

struct BoudingBox {
  min_corner: vec3<f32>,
  max_cornder: vec3<f32>,
  center: vec3<f32>,
  area: f32
}

// convert 2D seed to 1D
fn seed(x: u32, y: u32) -> u32 {
    return 19u * x + 47u * y + 101u;
}

// https://www.pcg-random.org/
fn pcg(v: u32) -> u32
{
  let state: u32 = v * 747796405u + 2891336453u;
	let word: u32 = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
	return (word >> 22u) ^ word;
}


// uses half open range [start, end)
struct Split {
  start: u32,
  end: u32,
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  // if negative, is leaf
  left_child: i32,
}

@compute @workgroup_size(1,1,1) 
fn find_splits(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {
    let segment_idx = id.x;
    let candidate_idx = id.y;
    let random_u32 = pcg(seed(segment_idx, candidate_idx));
    let random_idx = random_u32 % params.triangle_count;
    split_candidates[segment_idx * params.candidates_per_segment + candidate_idx] = triangle_info_buffer[random_idx].center;
}


