
//@group(0) @binding(0) var<uniform> scene_uniforms: SceneUniforms;

/*
struct SceneUniforms {
    @location(0) model_view: mat4x4<f32>,
  }
  */

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
   // out.clip_position = scene_uniforms.model_view * vec4<f32>(in.position, 1.0);
   out.clip_position = vec4<f32>(in.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
