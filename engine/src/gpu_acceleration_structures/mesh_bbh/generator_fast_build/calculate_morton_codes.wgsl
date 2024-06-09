
@group(0) @binding(0) var<storage, read> bb_buffer: array<BoundingBox>;
@group(0) @binding(1) var<storage, read> mesh_bb: BoundingBox;
@group(0) @binding(2) var<storage, read_write> morton_codes: array<u64>;

BoundingBox {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
}

struct MeshVertex {
  position: vec4<f32>,
  normal: vec4<f32>,
}

TODO: use wgsl ref and ptr

const BIGGEST = f32(1 << 19);
const BIGGEST_VEC = vec3<f32>(BIGGEST, BIGGEST, BIGGEST);
const TINY = 0.0000001f32;
const TINY_VEC = vec3<f32>(TINY, TINY, TINY);

// descretize into 20 integer bits per dimension
// TODO: Some of this can be pre computed on cpu once
fn discretize(v: vec3<f32>) -> vec3<f32> {
  // normalize
  let non_zero_size = mesh_bb.max_corner - mesh_bb.min_corner + TINY_VEC;
  v -= mesh_bb.min_corner;
  v /= non_zero_size;
  v *= BIGGEST_VEC;
  // discretize
  return vec3<u32>(v);
}

fn interlace(v: vec3<u32>) -> u64 {
  let bit = 0u32;
  let res = 0u64;
  for (var i = 0; i < 20; i++) {
    res |= (v.x & 1) << bit; 
    bit++;
    res |= (v.y & 1) << bit; 
    bit++;
    res |= (v.z & 1) << bit; 
    bit++;
    v >>= 1;
  }    
  return res;
}

fn calculate_morton_code(point: vec3<f32>) -> u64 {
  let discretized = descretize(point); 
  return interlace(discretized);
}

@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let bb = bb_buffer[id.x];
  let morton_code = calculate_morton_code((bb.min_corner + bb.max_corner) / 2.0);
  morton_codes[id.x] = morton_code;
}
