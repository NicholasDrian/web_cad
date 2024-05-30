// test a few splits to find the bes one
// currently using the surface area heuristic (SAH)

@group(0) @binding(0) var<storage, read> segments: array<Segment>;
@group(0) @binding(1) var<storage, read> bbh_index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read> triangle_info_buffer: array<TriangleInfo>;
@group(0) @binding(3) var<storage, read_write> splits: array<Split>;

struct TriangleInfo {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  center: vec3<f32>,
  area: f32,
  // 1 = true, 0 = false
  split_left: u32,
}

// convert 2D seed to 1D
fn seed(x: u32, y: u32) -> u32 {
    return 19u * x + 47u * y + 101u;
}

fn pcg(v: u32) -> u32
{
  let s: u32 = v * 747796405u + 2891336453u;
	let w: u32 = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
	return (w >> 22u) ^ w;
}


// uses half open range [start, end)
struct Segment {
  start: u32,
  end: u32,
}

struct Split {
    point: vec3<f32>,
    // split quality in each axis
    quality: vec3<f32>,
  }


@compute @workgroup_size(1,1,1) 
fn find_splits(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>,
  ) {
  
    let segment = segments[id.x];
    let random_u32 = pcg(seed(id.x, id.y));

    let segment_size = segment.end - segment.start;
    let candidate_idx = bbh_index_buffer[random_u32 % segment_size + segment.start];

    let candidate_center = triangle_info_buffer[candidate_idx].center;
    var sah_evaluation = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = segment.start; i < segment.end; i++) {
      let center = triangle_info_buffer[bbh_index_buffer[i]].center;
      let area = triangle_info_buffer[bbh_index_buffer[i]].area;

      // fancy sign() used to eliminate condition
      // double sign used to create -1 or 1
      // rather than -1, 0 or 1
      // s is 1 for left and -1 for right
      let s = sign(sign(candidate_center - center) + vec3<f32>(0.5, 0.5, 0.5));

      sah_evaluation += s * area; 
    }

    splits[id.x * size.y + id.y] = Split (
       candidate_center,
       abs(sah_evaluation)
    );

}


