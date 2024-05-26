@group(0) @binding(0) var<buffer, read> vertex_buffer: array<vec4<f32>>;
@group(0) @binding(1) var<buffer, read> index_buffer: array<u32>;
@group(0) @binding(2) var<buffer, read_write> bb_buffer: array<BoudingBox>;

struct BoundingBox {
  min: vec3<f32>;
  max: vec3<f32>;
  center: vec3<f32>;
  area: f32;
}


@compute @workgroup_size(1,1,1) 
fn split(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {
    let p1 = vertex_buffer[index_buffer[3 * id.x]];
    let p2 = vertex_buffer[index_buffer[3 * id.x + 1]];
    let p3 = vertex_buffer[index_buffer[3 * id.x + 2]];
    
    
    bb_buffer[id.x] = BoundingBox(
      vec3<f32>(
        0,
        0,
        0,
      ), 
      vec3<f32>(
        0,
        0,
        0,
      ), 
      vec3<f32>(
        0,
        0,
        0,
      ), 
    ); 

}
