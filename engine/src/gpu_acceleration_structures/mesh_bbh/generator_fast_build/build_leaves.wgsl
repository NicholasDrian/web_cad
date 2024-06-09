@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> bb_buffer: array<BoundingBox>;
@group(0) @binding(2) var<storage, read> bbh_index_buffer: array<u32>;
@group(0) @binding(3) var<storage, read_write> tree_buffer: array<Node>;

struct Params {
  tris_per_leaf: u32,
  tri_count: u32,
  offset: u32,
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
  let l = id * params.tris_per_leaf; 
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

  tree_buffer[params.offset + id.x] = Node(
    min_corner,
    max_corner,
    l,
    r,
    0,
  )

}

