
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> tree: array<Node>;
@group(0) @binding(2) var<storage, read_write> output: array<u32>;

struct Params {
  l: u32, 
  r: u32, 
  max_tris_per_leaf: u32,
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
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  // move this into for loop TODO
    var idx = 1;
    for (var i = params.l; i < params.r; i++) {
      output[idx] = output[idx - 1];
      let node = tree[i];
      if (node.r - node.l > params.max_tris_per_leaf) {
          output[idx] += 1;
      } 
      idx++;
    }
}

