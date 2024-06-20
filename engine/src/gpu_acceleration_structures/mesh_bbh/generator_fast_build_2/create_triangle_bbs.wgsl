@group(0) @binding(0) var<storage, read> vertex_buffer: array<Vertex>;
@group(0) @binding(1) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(2) var<storage, read_write> triangle_info: array<TriangleInfo>;

struct Vertex {
  position: vec4<f32>,
  normal: vec4<f32>,
}


struct TriangleInfo {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  surface_area: f32,
}

fn triangle_surface_area(a: vec3<f32>, b: vec3<f32>, c: vec3<f32>) -> f32 {
  return length(cross(a, b)) / 2.0;
}

@compute @workgroup_size(1,1,1) 
fn generate_bb_buffer(
  @builtin(global_invocation_id) id: vec3<u32>,
  ) {
    let p1 = vertex_buffer[index_buffer[3 * id.x]].position.xyz;
    let p2 = vertex_buffer[index_buffer[3 * id.x + 1]].position.xyz;
    let p3 = vertex_buffer[index_buffer[3 * id.x + 2]].position.xyz;
    
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

    let surface_area = triangle_surface_area(p1, p2, p3);

    triangle_info[id.x] = TriangleInfo(
        min_corner,
        max_corner,
      surface_area,
    ); 

}
