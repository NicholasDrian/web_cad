@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> bb_buffer: array<BoundingBox>;
@group(0) @binding(2) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(3) var<storage, read_write> tree_buffer: array<Node>;

struct Params {
  tris_per_leaf: u32,
  node_count: u32,
  leaf_count: u32,
  tri_count: u32,
  first_bottom_idx: u32, // Idx of first bottom row node
}

struct BoundingBox {
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

@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {

  var idx = id.x + params.first_bottom_idx;

  // node is not on last row
  if (idx >= params.node_count) {
      idx -= params.leaf_count;
  }
  
  let l = idx * params.tris_per_leaf; 
  // TODO: could get rid of this min if im cleaver
  let r = min(l + params.tris_per_leaf, params.tri_count);

  let first_bb = bb_buffer[index_buffer[l]];
  var min_corner = first_bb.min_corner;
  var max_corner = first_bb.max_corner;

  for (var i = l + 1; i < r; i++) {
      let bb = bb_buffer[index_buffer[i]];
      min_corner = min(min_corner, bb.min_corner);
      max_corner = max(max_corner, bb.max_corner);
  }

  tree_buffer[idx] = Node(
    min_corner,
    max_corner,
    l,
    r,
    0,
  );

}

