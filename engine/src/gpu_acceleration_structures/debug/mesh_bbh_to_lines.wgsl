@group(0) @binding(0) var<storage, read> tree: array<Node>;
@group(0) @binding(1) var<storage, read_write> vertex_buffer: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> index_buffer: array<u32>;


struct Node {
  min_corner: vec3<f32>,
  l: u32,
  max_corner: vec3<f32>,
  r: u32,
  center: vec3<f32>,
  left_child: u32,
}

@compute @workgroup_size(1,1,1)
fn main(
  @builtin(global_invocation_id) id: vec3<u32>
  ) {

    let node = tree[id.x];
    let min_corner = node.min_corner;
    let max_corner = node.max_corner;

    let idx0 = id.x * 8 + 0;
    let idx1 = id.x * 8 + 1;
    let idx2 = id.x * 8 + 2;
    let idx3 = id.x * 8 + 3;
    let idx4 = id.x * 8 + 4;
    let idx5 = id.x * 8 + 5;
    let idx6 = id.x * 8 + 6;
    let idx7 = id.x * 8 + 7;
    
    vertex_buffer[idx0] = vec4<f32>(min_corner.x, min_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx1] = vec4<f32>(min_corner.x, min_corner.y, max_corner.z, 1.0);
    vertex_buffer[idx2] = vec4<f32>(min_corner.x, max_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx3] = vec4<f32>(min_corner.x, max_corner.y, max_corner.z, 1.0);
    vertex_buffer[idx4] = vec4<f32>(max_corner.x, min_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx5] = vec4<f32>(max_corner.x, min_corner.y, max_corner.z, 1.0);
    vertex_buffer[idx6] = vec4<f32>(max_corner.x, max_corner.y, min_corner.z, 1.0);
    vertex_buffer[idx7] = vec4<f32>(max_corner.x, max_corner.y, max_corner.z, 1.0);

    index_buffer[id.x * 24 + 0] = idx0;
    index_buffer[id.x * 24 + 1] = idx1;
    index_buffer[id.x * 24 + 2] = idx0;
    index_buffer[id.x * 24 + 3] = idx2;
    index_buffer[id.x * 24 + 4] = idx0;
    index_buffer[id.x * 24 + 5] = idx4;

    index_buffer[id.x * 24 + 6] = idx3;
    index_buffer[id.x * 24 + 7] = idx1;
    index_buffer[id.x * 24 + 8] = idx3;
    index_buffer[id.x * 24 + 9] = idx2;
    index_buffer[id.x * 24 + 10] = idx3;
    index_buffer[id.x * 24 + 11] = idx7;

    index_buffer[id.x * 24 + 12] = idx5;
    index_buffer[id.x * 24 + 13] = idx1;
    index_buffer[id.x * 24 + 14] = idx5;
    index_buffer[id.x * 24 + 15] = idx4;
    index_buffer[id.x * 24 + 16] = idx5;
    index_buffer[id.x * 24 + 17] = idx7;

    index_buffer[id.x * 24 + 18] = idx6;
    index_buffer[id.x * 24 + 19] = idx2;
    index_buffer[id.x * 24 + 20] = idx6;
    index_buffer[id.x * 24 + 21] = idx4;
    index_buffer[id.x * 24 + 22] = idx6;
    index_buffer[id.x * 24 + 23] = idx7;

  }
