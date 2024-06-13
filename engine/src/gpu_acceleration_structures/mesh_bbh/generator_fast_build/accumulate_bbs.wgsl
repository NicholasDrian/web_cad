
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read_write> bb_buffer: array<BoundingBox>;

struct Params {
  offset: u32,
}

struct BoundingBox {
    min_corner: vec3<f32>,
    max_corner: vec3<f32>,
}

// todo:
// can i pass by ref?
// is the copy elided?
fn add_bbs(a: BoundingBox, b: BoundingBox) -> BoundingBox {
  return BoundingBox (
    min(a.min_corner, b.min_corner),
    max(a.max_corner, b.max_corner)
  );
}


@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let dst = id.x * params.offset * 2;
  let src = dst + params.offset;
  bb_buffer[dst] = add_bbs(bb_buffer[src], bb_buffer[dst]); 
}
