@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read> triangle_bbs: array<BoundingBox>;
@group(0) @binding(3) var<storage, read> tree: array<Node>;
@group(0) @binding(4) var<storage, read_write> split_evaluations: array<SplitEval>;

struct Params {
    offset: u32,
    max_tris_per_leaf: u32,
    split_candidates: u32,
  }

struct SplitEval {
    point: vec3<f32>,
    // split quality in each axis
    // todo: Should take into account distance of child bb centers
    quality: vec3<u32>,
  }


@compute @workgroup_size(1,1,1)
fn main() {}
