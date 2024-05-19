
@group(0) @binding(0) var<uniform> scene_uniforms: SceneUniforms;
@group(1) @binding(0) var<uniform> geometry_uniforms: GeometryUniforms;


struct SceneUniforms {
    view_proj: mat4x4<f32>,
  }

struct GeometryUniforms {
    model: mat4x4<f32>,
    color: vec4<f32>,
  }


struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec4<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = in.normal;
    out.clip_position = scene_uniforms.view_proj * geometry_uniforms.model * in.position;
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.normal.xyz / 2.0 + vec3<f32>(0.5, 0.5, 0.5), 1.0);
}
