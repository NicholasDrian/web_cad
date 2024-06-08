
@group(0) @binding(0) var<uniform> params: array<Params>;
@group(0) @binding(0) var<buffer, read> bbh_index_buffer: array<u32>;
@group(0) @binding(0) var<buffer, read> bb_buffer: array<BoundingBox>;
@group(0) @binding(0) var<buffer, read> tree: array<Node>;

struct Params {
    tris_per_leaf: u32,
    branch_factor: u32,
    level: u32,
}

struct Node {
    min_corner: vec3<f32>,
    // not needed for this bbh because a nodes indices correspond with the nodes position
    // needed for other bbhs where this is not the case
    l: u32,

    max_corner: vec3<f32>,

    // not needed for this bbh
    r: u32,

    // should probably get rid of this param globally
    center: vec3<f32>,

    // not needed for this bbh
    left_child: u32,
  }

struct BoundingBox {
    min_corner: vec3<f32>,
    max_corner: vec3<f32>,
  }


fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>,
) {
    ///todo!()
}

