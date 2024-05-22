use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::render::renderer::Renderer;

use super::bind_group::GeometryBindGroupObject;
use crate::geometry::Geometry;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PolylineVertex {
    pub position: [f32; 4],
}

pub static POLYLINE_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> =
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<PolylineVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x4,
        }],
    };

pub struct Polyline {
    vertex_buffer: wgpu::Buffer,
    bind_group_object: GeometryBindGroupObject,
    vertex_count: u32,
}

impl Polyline {
    pub fn new(renderer: Rc<Renderer>, verts: &[PolylineVertex]) -> Polyline {
        let vertex_buffer =
            renderer
                .get_device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let bind_group_object = GeometryBindGroupObject::new(renderer);
        Polyline {
            vertex_buffer,
            vertex_count: verts.len() as u32,
            bind_group_object,
        }
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        self.bind_group_object.get_bind_group()
    }
}

impl Geometry for Polyline {
    fn get_bind_group_object_mut(&mut self) -> &mut GeometryBindGroupObject {
        &mut self.bind_group_object
    }
}
