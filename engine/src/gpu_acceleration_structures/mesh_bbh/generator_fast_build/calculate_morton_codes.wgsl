
@group(0) @binding(0) var<storage, read> bb_buffer: array<BoundingBox>;
@group(0) @binding(1) var<storage, read> mesh_bb: BoundingBox;
@group(0) @binding(2) var<storage, read_write> morton_codes: array<MyU64>;

struct BoundingBox {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
}

struct MyU64 {
  upper_bits: u32,
  lower_bits: u32,
}

struct MeshVertex {
  position: vec4<f32>,
  normal: vec4<f32>,
}

//TODO: use wgsl ref and ptr

const BIGGEST = f32(1 << 19);
const BIGGEST_VEC = vec3<f32>(BIGGEST, BIGGEST, BIGGEST);
const TINY = 0.0000001;
const TINY_VEC = vec3<f32>(TINY, TINY, TINY);

// descretize into 20 integer bits per dimension
// TODO: Some of this can be pre computed on cpu once
fn discretize(v: vec3<f32>) -> vec3<u32> {
  // normalize
  var v_clone = v;
  let non_zero_size = mesh_bb.max_corner - mesh_bb.min_corner + TINY_VEC;
  v_clone -= mesh_bb.min_corner;
  v_clone /= non_zero_size;
  v_clone *= BIGGEST_VEC;
  // discretize
  return vec3<u32>(v_clone);
}

fn interlace(v: vec3<u32>) -> MyU64 {
  var v_clone = v;
  var upper_bits = 0u;
  var lower_bits = 0u;
  var bit = 0u;
  for (var i = 0; i < 10; i++) {
    lower_bits |= (v_clone.x & 1) << bit; 
    bit++;
    lower_bits |= (v_clone.y & 1) << bit; 
    bit++;
    lower_bits |= (v_clone.z & 1) << bit; 
    bit++;
    v_clone = v_clone >> vec3<u32>(1,1,1);
  }    
  bit = 0u;
  for (var i = 0; i < 10; i++) {
    upper_bits |= (v_clone.x & 1) << bit; 
    bit++;
    upper_bits |= (v_clone.y & 1) << bit; 
    bit++;
    upper_bits |= (v_clone.z & 1) << bit; 
    bit++;
    v_clone = v_clone >> vec3<u32>(1,1,1);
  }    
  return MyU64(upper_bits, lower_bits);
}

fn calculate_morton_code(point: vec3<f32>) -> MyU64 {
  let discretized = discretize(point); 
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
