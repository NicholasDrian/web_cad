@group(0) @binding(0) var<buffer, read> vertex_buffer: array<vec4<f32>>;
@group(0) @binding(1) var<buffer, read> index_buffer: array<u32>;
@group(0) @binding(2) var<buffer, read_write> bb_buffer: array<BoudingBox>;

struct BoundingBox {
  min_corner: vec3<f32>;
  max_corner: vec3<f32>;
  center: vec3<f32>;
  area: f32;
}


@compute @workgroup_size(1,1,1) 
fn generate_bb_buffer(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {
    let p1 = vertex_buffer[index_buffer[3 * id.x]];
    let p2 = vertex_buffer[index_buffer[3 * id.x + 1]];
    let p3 = vertex_buffer[index_buffer[3 * id.x + 2]];
    
    let min_corner = vec3<f32>(
        min(min(p1.x, p2.x), p3.x),
        min(min(p1.y, p2.y), p3.y),
        min(min(p1.z, p2.z), p3.z),
      );
    let max_corner = vec3<f32>(
        max(max(p1.x, p2.x), p3.x),
        max(max(p1.y, p2.y), p3.y),
        max(max(p1.z, p2.z), p3.z),
      ), 
    let center = (min_corner + max_corner) / 2.0;
    
    bb_buffer[id.x] = BoundingBox(
      min_corner,
      max_corner,
      center
    ); 

}
