use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{
    math::linear_algebra::{mat4::Mat4, vec3::Vec3, vec4::Vec4},
    render::renderer::Renderer,
};

use super::geometry::{Geometry, GeometryUniforms};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
}

pub static MESH_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> =
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x4,
            },
        ],
    };

pub struct Mesh {
    renderer: Rc<Renderer>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    index_count: u32,
    model: Mat4,
}

impl Mesh {
    pub fn new(renderer: Rc<Renderer>, verts: &[MeshVertex], indices: &[u32]) -> Mesh {
        let vertex_buffer =
            renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    usage: wgpu::BufferUsages::INDEX,
                    contents: bytemuck::cast_slice(indices),
                });
        let model = Mat4::identity();
        let uniform_buffer =
            renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("mesh uniform buffer"),
                    contents: bytemuck::cast_slice(&[GeometryUniforms {
                        model,
                        color: Vec4 {
                            x: 0.0,
                            y: 0.5,
                            z: 1.0,
                            w: 1.0,
                        },
                    }]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let bind_group = renderer
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("mesh bind group"),
                layout: renderer.get_geometry_bind_group_layout(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });
        Mesh {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
            index_count: indices.len() as u32,
            model,
            renderer,
        }
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn get_index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn get_index_count(&self) -> u32 {
        self.index_count
    }
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

impl Geometry for Mesh {
    fn rotate(&mut self, center: Vec3, axis: Vec3, radians: f32) {
        let rotation = Mat4::rotate_center_axis(center, axis, radians);
        self.model = Mat4::multiply(&rotation, &self.model);
        self.renderer.get_queue().write_buffer(
            &self.uniform_buffer,
            std::mem::offset_of!(GeometryUniforms, model) as u64,
            bytemuck::cast_slice(&[self.model]),
        );
    }
}
