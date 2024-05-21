use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{
    math::linear_algebra::{mat4::Mat4, vec3::Vec3, vec4::Vec4},
    render::renderer::Renderer,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GeometryUniforms {
    pub model: Mat4,
    pub color: Vec4,
}

pub struct GeometryBindGroupObject {
    renderer: Rc<Renderer>,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    model: Mat4,
    color: Vec4,
}

impl GeometryBindGroupObject {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let model = Mat4::identity();
        let color = Vec4 {
            x: 0.0,
            y: 0.5,
            z: 1.0,
            w: 1.0,
        };
        let buffer = renderer
            .get_device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("mesh uniform buffer"),
                contents: bytemuck::cast_slice(&[GeometryUniforms { model, color }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group = renderer
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("mesh bind group"),
                layout: renderer.get_geometry_bind_group_layout(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });
        Self {
            renderer,
            buffer,
            bind_group,
            color,
            model,
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn rotate(&mut self, center: Vec3, axis: Vec3, radians: f32) {
        let rotation = Mat4::rotate_center_axis(center, axis, radians);
        self.model = Mat4::multiply(&rotation, &self.model);
        self.renderer.get_queue().write_buffer(
            &self.buffer,
            std::mem::offset_of!(GeometryUniforms, model) as u64,
            bytemuck::cast_slice(&[self.model]),
        );
    }
}
