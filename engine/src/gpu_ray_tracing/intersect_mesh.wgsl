@group(0) @binding(0) var<storage, read> bbh: array<Node>;
@group(0) @binding(1) var<storage, read> vertex_buffer: array<Vertex>;
@group(0) @binding(2) var<storage, read> index_buffer: array<u32>;
@group(0) @binding(3) var<storate, read> rays: array<Ray>;

struct Ray {
  // Maybe check alignment rules
  position: Vec3,
  direction: Vec3
}

struct Vertex {
  position: Vec4,
  normal: Vec4, 
}

struct Node {
  min_corner: vec3<f32>,
  l: u32,
  max_corner: vec3<f32>,
  r: u32,
  center: vec3<f32>,
  left_child: u32,
}

fn intersect_bounding_box(ray: Ray, bb: BoundingBox) -> some shit {

}

fn intersect_tri(ray: Ray, tri: Tri) -> some shit {

}

@compute @workgroup_size(1,1,1) 
fn main(
  @builtin(global_invocation_id) id: vec3<u32>,
  @builtin(num_workgroups) num_workgroups: vec3<u32>
  ) {

}
