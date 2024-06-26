use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{gpu_acceleration_structures::mesh_bbh::MeshBBH, render::renderer::Renderer};

use super::{bind_group::GeometryBindGroupObject, Geometry};

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
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group_object: GeometryBindGroupObject,
    index_count: u32,
    vertex_count: u32,
    bbh: Option<MeshBBH>,
}

impl Mesh {
    pub fn new(renderer: Rc<Renderer>, verts: &[MeshVertex], indices: &[u32]) -> Mesh {
        let vertex_buffer =
            renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(verts),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_SRC,
                });
        let index_buffer =
            renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_SRC,
                    contents: bytemuck::cast_slice(indices),
                });
        let bind_group = GeometryBindGroupObject::new(renderer);
        Mesh {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            vertex_count: verts.len() as u32,
            bind_group_object: bind_group,
            bbh: None,
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
    pub fn add_bbh(&mut self, bbh: MeshBBH) -> &mut Self {
        self.bbh = Some(bbh);
        self
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }
}

impl Geometry for Mesh {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject {
        &mut self.bind_group_object
    }
}
