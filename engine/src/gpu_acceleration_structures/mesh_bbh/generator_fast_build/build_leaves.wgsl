@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> bb_buffer: array<BoundingBox>;
@group(0) @binding(2) var<storage, read> bbh_index_buffer: array<u32>;
@group(0) @binding(3) var<storage, read_write> tree_buffer: array<Node>;

struct Params {
  tris_per_leaf: u32,
  node_count: u32,
  tri_count: u32,
  first_leaf_idx: u32, // Idx of first leaf node
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


  // this seems iffy
  // calculate idx of node as if its on the bottom row
  var idx = id.x + first_bottom_idx;
  // if idx is invalid, re calculate it as if node is on second to last row
  if (idx >= node_count) {
      idx = first_leaf_idx + id.x - (node_count - first_bottom_idx);
  }
  
  // TODO leaves are in wrong order

  let l = idx * params.tris_per_leaf; 
  let r = l + params.tris_per_leaf;
  r = min(r, params.tri_count);

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
  )

}

