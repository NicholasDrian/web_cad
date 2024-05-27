// test a few splits to find the bes one
// currently using the surface area heuristic (SAH)

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> bb_buffer: array<BoudingBox>;
@group(0) @binding(2) var<storage, read> splits_in: array<Split>;
@group(0) @binding(3) var<storage, read_write> splits_out: array<SplitCandidate>;


struct Params {
  // TODO:
  temp: u32,
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

struct SplitCandidate {
  start: u32,
  end: u32,
  // for SAH
  // the difference in cumulative surface area between each side of split
  // could be negative
  diff: i32,
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  // if negative, is leaf
  left_child: i32,
}

// test x y and z splits in each work group
@compute @workgroup_size(3,1,1) 
fn find_splits(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {

}


