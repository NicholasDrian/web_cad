@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> tree: array<Node>;
@group(0) @binding(2) var<storage, read_write> vertex_buffer: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> index_buffer: array<u32>;

struct Params {
    max_tris_per_leaf: u32
}

struct Node {
  min_corner: vec3<f32>,
  max_corner: vec3<f32>,
  l: u32,
  r: u32,
  left_child: u32,
}

@compute @workgroup_size(8,8,4)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) size: vec3<u32>
  ) {

    let idx = 
      id.x + 
      id.y * size.x + 
      id.z * size.x * size.y;

    let node = tree[idx];

    // use this to only show leaf boxes
    if (node.r - node.l > params.max_tris_per_leaf) {
      return;
    }

    let min_corner = node.min_corner;
    let max_corner = node.max_corner;

    let idx0 = idx * 8 + 0;
    let idx1 = idx * 8 + 1;
    let idx2 = idx * 8 + 2;
    let idx3 = idx * 8 + 3;
    let idx4 = idx * 8 + 4;
    let idx5 = idx * 8 + 5;
    let idx6 = idx * 8 + 6;
    let idx7 = idx * 8 + 7;
    
    vertex_buffer[idx0] = vec4<f32>(min_corner.x, min_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx1] = vec4<f32>(min_corner.x, min_corner.y, max_corner.z, 1.0);
    vertex_buffer[idx2] = vec4<f32>(min_corner.x, max_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx3] = vec4<f32>(min_corner.x, max_corner.y, max_corner.z, 1.0);
    vertex_buffer[idx4] = vec4<f32>(max_corner.x, min_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx5] = vec4<f32>(max_corner.x, min_corner.y, max_corner.z, 1.0);
    vertex_buffer[idx6] = vec4<f32>(max_corner.x, max_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx7] = vec4<f32>(max_corner.x, max_corner.y, max_corner.z, 1.0);

    index_buffer[idx * 24 + 0] = idx0;
    index_buffer[idx * 24 + 1] = idx1;
    index_buffer[idx * 24 + 2] = idx0;
    index_buffer[idx * 24 + 3] = idx2;
    index_buffer[idx * 24 + 4] = idx0;
    index_buffer[idx * 24 + 5] = idx4;

    index_buffer[idx * 24 + 6] = idx3;
    index_buffer[idx * 24 + 7] = idx1;
    index_buffer[idx * 24 + 8] = idx3;
    index_buffer[idx * 24 + 9] = idx2;
    index_buffer[idx * 24 + 10] = idx3;
    index_buffer[idx * 24 + 11] = idx7;

    index_buffer[idx * 24 + 12] = idx5;
    index_buffer[idx * 24 + 13] = idx1;
    index_buffer[idx * 24 + 14] = idx5;
    index_buffer[idx * 24 + 15] = idx4;
    index_buffer[idx * 24 + 16] = idx5;
    index_buffer[idx * 24 + 17] = idx7;

    index_buffer[idx * 24 + 18] = idx6;
    index_buffer[idx * 24 + 19] = idx2;
    index_buffer[idx * 24 + 20] = idx6;
    index_buffer[idx * 24 + 21] = idx4;
    index_buffer[idx * 24 + 22] = idx6;
    index_buffer[idx * 24 + 23] = idx7;

  }
