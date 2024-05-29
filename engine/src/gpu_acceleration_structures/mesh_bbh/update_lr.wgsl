@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> segments: array<Segment>;
@group(0) @binding(2) var<storage, read> bbh_index_buffer: array<u32>;
@group(0) @binding(3) var<storage, read> splits: array<Split>;
@group(0) @binding(4) var<storage, read_write> triangle_info_buffer: array<TriangleInfo>;

struct Params {
  candidates_per_segment: u32    
}

struct TriangleInfo {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  center: vec3<f32>,
  area: f32,
  // 1 = true, 0 = false
  split_left: u32,
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
fn update_lr(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {
    let segment = segments[id.x]; 

    var best_point = vec3<f32>(0.0, 0.0, 0.0);
    var best_dir = 0;
    for (var i = 0u; i < params.candidates_per_segment; i++) {

    }

}


