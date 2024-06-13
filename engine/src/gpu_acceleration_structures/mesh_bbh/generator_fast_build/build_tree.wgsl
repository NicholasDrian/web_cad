//TODO: variable branch factor?

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> tree: array<Node>;

struct Params {
    offset: u32,
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
  let idx = id.x + params.offset;
  let left_child_idx = 2 * idx + 1;
  let left_child = tree[left_child_idx];
  let right_child = tree[left_child_idx + 1];
  let min_corner = min(left_child.min_corner, right_child.min_corner);
  let max_corner = max(left_child.max_corner, right_child.max_corner);
  let l = left_child.l;
  let r = right_child.r;
  tree[idx] = Node (
    min_corner, 
    max_corner, 
    l,
    r,
    left_child_idx
  );
}

