@group(0) @binding(0) var<storage, read> vertex_buffer: array<Vertex>;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read_write> bb_buffer: array<BoundingBox>;

struct Vertex {
  position: vec4<f32>,
  normal: vec4<f32>,
}

struct BoundingBox {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  center: vec3<f32>,
}

@compute @workgroup_size(1,1,1) 
fn generate_bb_buffer(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {
    let p1 = vertex_buffer[index_buffer[3 * id.x]].position;
    let p2 = vertex_buffer[index_buffer[3 * id.x + 1]].position;
    let p3 = vertex_buffer[index_buffer[3 * id.x + 2]].position;
    
    let min_corner = vec3<f32>(
        min(min(p1.x, p2.x), p3.x),
        min(min(p1.y, p2.y), p3.y),
        min(min(p1.z, p2.z), p3.z),
      );
    let max_corner = vec3<f32>(
        max(max(p1.x, p2.x), p3.x),
        max(max(p1.y, p2.y), p3.y),
        max(max(p1.z, p2.z), p3.z),
      );
    let center = (min_corner + max_corner) / 2;

    bb_buffer[id.x] = BoundingBox(
      min_corner,
      max_corner,
      center
    ); 

}
fn calculate_morton_code(point: vec3<f32>) -> u64 {
  // descretize into 20 integer bits
  let max_size = params.

  // interlace the bits

}

