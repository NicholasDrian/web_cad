TODO: variable branch factor?

@group(0) @binding(0) var<uniform> params: array<Params>;
@group(0) @binding(1) var<buffer, read> tree: array<Node>;

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

fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let left_child = tree[2 * id.x + 1];
  let right_child = tree[2 * id.x + 2];
  let min_corner = min(left_child.min_corner, right_child.min_corner);
  let max_corner = max(left_child.max_corner, right_child.max_corner);
  let l = left_child.l;
  let r = right_child.r;
  let left_child = 2 * id.x + 1;
  let tree[offset + id.x] = Node (
    min_corner, 
    max_corner, 
    l,
    r,
    left_child
  );
}

