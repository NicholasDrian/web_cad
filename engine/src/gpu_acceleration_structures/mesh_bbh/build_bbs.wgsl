@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read> triangle_bbs: array<BoundingBox>;
@group(0) @binding(3) var<storage, read_write> tree: array<Node>;


struct Params {
  offset: u32
}

struct BoundingBox {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  center: vec3<f32>,
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
fn build_bbs(
  @builtin(global_invocation_id) id: vec3<u32>,
) {

  let node = tree[params.offset + id.x];

  var min_corner = triangle_bbs[index_buffer[node.l]].min_corner;
  var max_corner = triangle_bbs[index_buffer[node.l]].max_corner;

  for (var i = node.l + 1; i < node.r; i++) {
    min_corner = min(min_corner, triangle_bbs[index_buffer[i]].min_corner);
    max_corner = max(max_corner, triangle_bbs[index_buffer[i]].max_corner);
  }

  tree[params.offset + id.x].min_corner = min_corner;
  tree[params.offset + id.x].max_corner = max_corner;
    
}


