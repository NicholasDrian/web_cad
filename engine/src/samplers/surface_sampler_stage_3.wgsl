// Calculate normals

@group(0) @binding(1) var<storage, read_write> vertex_buffer: array<vec4<f32>>;

@compute @workgroup_size(1,1,1) 
  fn main(
      @builtin(global_invocation_id) id: vec3<u32>,
      @builtin(num_workgroups) size: vec3<u32>
      ) {

    var offset_x: i32;
    var offset_y: i32;

    if (id.x == 0) {
      offset_x = 2;
    } else {
      offset_x = -2;
    }

    var offset_y;
    if (id.y == 0) {
      offset_y = i32(size.x) * 2;
    } else {
      offset_y = i32(size.x) * -2;
    }

    let idx_point: i32 = (id.x + id.y * size.x) * 2;
    let idx_point_dx: i32 = idx_point + offset_x;
    let idx_point_dy: i32 = idx_point + offset_y;

    let point = vertex_buffer[u32(idx_point)];
    let point_dx = vertex_buffer[u32(idx_point_dx)];
    let point_dy = vertex_buffer[u32(idx_point_dy)];

// TODO: think about w...
// TEST

    let dx = point_dx - point;
    let dy = point_dy - point;

    let normal = normaize(cross(dx, dy));
    
    samples[u32(idx_point) + 1] = normal;

  }



