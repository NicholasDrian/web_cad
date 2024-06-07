// calculate triangle bbh and morton code

@group(0) @binding(1) var<storage, read_write> triangle_info_buffer: array<TriangleInfo>;

// could probably pack this better
struct TriangleInfo {
    min_corner: vec3<f32>,
    max_corner: vec3<f32>,
    morton_code: u64,
  }

fn calculate_morton_code(point: vec3<f32>) -> u64 {
  // descretize into 20 integer bits
}
