@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> triangle_info: array<TriangleInfo>;
@group(0) @binding(2) var<storage, read> split_evaluations: array<SplitEval>;
@group(0) @binding(3) var<storage, read_write> index_buffer: array<u32>;
@group(0) @binding(4) var<storage, read_write> tree: array<Node>;

struct Params {
  offset: u32,
  max_tris_per_leaf: u32,
  split_candidates: u32,
  this_level_node_count: u32,
}

struct SplitEval {
  point: vec3<f32>,
  // split quality in each axis
  quality: vec3<f32>,
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

fn swap_indices(i: u32, j: u32) {
  let temp = index_buffer[i];
  index_buffer[i] = index_buffer[j];
  index_buffer[j] = temp;
}

const FLOAT_MAX = 3.40282346638528859812e+38f;

@compute @workgroup_size(1,1,1)
fn build_next_level(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>,
) {

  let idx = 
    id.x + 
    id.y * size.x + 
    id.z * size.x * size.y;

TODO: add this check elsewhere in this module
  if (idx >= params.this_level_node_count) { return; }

  let node = tree[idx + params.offset];
  let span = node.r - node.l;



  if (span <= params.max_tris_per_leaf) {
    // is leaf, no need to build next level
    return;
  }


    // figure out what split was best
    var best_point = vec3<f32>(0.0, 0.0, 0.0);
    var best_dir = 0u;
    var best_sah = 0.0;
    for (var i = 0u; i < params.split_candidates; i++) {
      let split_eval = split_evaluations[idx * params.split_candidates + i];
      if (split_eval.quality.x > best_sah) {
        best_point = split_eval.point;
        best_sah = split_eval.quality.x;
        best_dir = 0u;
      } 
      if (split_eval.quality.y > best_sah) {
        best_point = split_eval.point;
        best_sah = split_eval.quality.y;
        best_dir = 1u;
      } 
      if (split_eval.quality.z > best_sah) {
        best_point = split_eval.point;
        best_sah = split_eval.quality.z;
        best_dir = 2u;
      } 
    }

    // reorder the indices in place
    var low = node.l;
    var high = node.l;

    while (high < node.r) {

      let info = triangle_info[index_buffer[high]];
      let center = (info.min_corner + info.max_corner) / 2.0;
      let delta = center - best_point; 

      var is_left = false;
      if (best_dir == 0) {
        is_left = delta.x > 0.0;
      } else if (best_dir == 1) {
        is_left = delta.y > 0.0;
      } else {
        is_left = delta.z > 0.0;
      }

      if (is_left) {
        swap_indices(low, high);
        low++;
      } 
      high++;

    }

  // set child pointers
  let left_child_idx = (params.offset + idx) * 2 + 1; 
  tree[idx + params.offset].left_child = left_child_idx;


  // write out next level
  tree[left_child_idx] = Node(
    vec3<f32>(0.0, 0.0, 0.0),
    vec3<f32>(0.0, 0.0, 0.0),
    node.l,
    low,
    0,
  );

  tree[left_child_idx + 1] = Node(
    vec3<f32>(0.0, 0.0, 0.0),
    vec3<f32>(0.0, 0.0, 0.0),
    low,
    node.r,
    0,
  );

}
