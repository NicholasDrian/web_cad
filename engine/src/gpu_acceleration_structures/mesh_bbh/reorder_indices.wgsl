@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> segments: array<Segment>;
@group(0) @binding(2) var<storage, read> splits: array<Split>;
@group(0) @binding(3) var<storage, read_write> bbh_index_buffer: array<u32>;
@group(0) @binding(4) var<storage, read> triangle_info_buffer: array<TriangleInfo>;
@group(0) @binding(5) var<storage, read_write> new_segments: array<Segment>;

struct Params {
candidates_per_segment: u32    
}

struct TriangleInfo {
min_corner: vec3<f32>,
              max_corner: vec3<f32>,
              center: vec3<f32>,
              area: f32,
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

fn swap_indices(i: u32, j: u32) {
  let temp = bbh_index_buffer[i];
  bbh_index_buffer[i] = bbh_index_buffer[j];
  bbh_index_buffer[j] = temp;
}

// I dont think the lang has this builtin? maybe im wrong
const FLOAT_MAX = 3.40282346638528859812e+38f;

@compute @workgroup_size(1,1,1) 
  fn update_lr(
      @builtin(global_invocation_id) id: vec3<u32>,
      ) {

    // figure out what split was best
    var best_point = vec3<f32>(0.0, 0.0, 0.0);
    var best_dir = 0u;
    var best_sah = FLOAT_MAX;
    for (var i = 0u; i < params.candidates_per_segment; i++) {
      let split = splits[id.x * params.candidates_per_segment + i];
      if (split.quality.x < best_sah) {
        best_point = split.point;
        best_sah = split.quality.x;
        best_dir = 0u;
      } 
      if (split.quality.y < best_sah) {
        best_point = split.point;
        best_sah = split.quality.y;
        best_dir = 1u;
      } 
      if (split.quality.z < best_sah) {
        best_point = split.point;
        best_sah = split.quality.z;
        best_dir = 2u;
      } 
    }
    /*
     * pseudo code
       int low = 0;
       int mid = 0;    
       while (mid < v.size()) {
       if (v[mid] == 0) {
       swap(v[low], v[mid]);
       low++;
       mid++;
       } else if (v[mid] == 1) {
       mid++;
       }
     */


    // reorder the indices in place
    let segment = segments[id.x]; 
    var low = segment.start;
    var high = segment.start;
    var left_count = 0;


    while (high < segment.end) {

      let point = triangle_info_buffer[bbh_index_buffer[high]].center;
      let delta = point - best_point; 

      var is_left = false;
      // could use enum for directions
      if (best_dir == 0) {
        is_left = delta.x > 0.0;
      } else if (best_dir == 0) {
        is_left = delta.y > 0.0;
      } else {
        is_left = delta.z > 0.0;
      }

      if (is_left) {
        left_count++;
        swap_indices(low, high);
        low++;
        high++;
      } else {
          high++;
      }

    }

    new_segments[id.x * 2] = Segment(segment.start, left_count);
    new_segments[id.x * 2 + 1] = Segment(left_count, segment.end);

  }


