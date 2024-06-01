use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::render::renderer::Renderer;

use super::bind_group::GeometryBindGroupObject;
use crate::geometry::Geometry;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LinesVertex {
    pub position: [f32; 4],
}

pub static LINES_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> =
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<LinesVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x4,
        }],
    };

pub struct Lines {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group_object: GeometryBindGroupObject,
    index_count: u32,
}

impl Lines {
    pub fn new(renderer: Rc<Renderer>, verts: &[LinesVertex], indices: &[u32]) -> Lines {
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
                    label: Some("index buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });
        let bind_group_object = GeometryBindGroupObject::new(renderer);
        Lines {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            bind_group_object,
        }
    }

    pub fn from_buffers(
        renderer: Rc<Renderer>,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        index_count: u32,
    ) -> Self {
        let bind_group_object = GeometryBindGroupObject::new(renderer);
        Self {
            vertex_buffer,
            index_buffer,
            bind_group_object,
            index_count,
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
        self.bind_group_object.get_bind_group()
    }
}

impl Geometry for Lines {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject {
        &mut self.bind_group_object
    }
}
