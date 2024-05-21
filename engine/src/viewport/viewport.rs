use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::math::linear_algebra::mat4::Mat4;
use crate::render::renderer::Renderer;

use super::camera::{Camera, CameraDescriptor};
use web_sys::HtmlCanvasElement;

pub struct ViewportInternal {
    camera: Camera,
    canvas: HtmlCanvasElement,
    color_surface: wgpu::Surface<'static>,
    bind_group: wgpu::BindGroup,
    depth_texture: wgpu::Texture,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewportUniforms {
    view_proj: Mat4,
}

impl ViewportInternal {
    pub fn new(canvas: HtmlCanvasElement, renderer: Rc<Renderer>) -> ViewportInternal {
        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.clone());
        let surface = renderer
            .get_instance()
            .create_surface(surface_target)
            .unwrap();
        let config = surface
            .get_default_config(renderer.get_adapter(), canvas.width(), canvas.height())
            .unwrap();

        surface.configure(renderer.get_device(), &config);

        let camera = Camera::new(CameraDescriptor::default(), renderer.clone());

        let bind_group = renderer
            .get_device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("scene bind group"),
                layout: renderer.get_viewport_bind_group_layout(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera.get_view_proj_buffer().as_entire_binding(),
                }],
            });

        let depth_texture = renderer
            .get_device()
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("depth texture"),
                size: wgpu::Extent3d {
                    width: canvas.width(),
                    height: canvas.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

        ViewportInternal {
            camera,
            canvas,
            color_surface: surface,
            bind_group,
            depth_texture,
        }
    }

    pub fn get_views(&self) -> (wgpu::TextureView, wgpu::TextureView) {
        let view_color = self
            .color_surface
            .get_current_texture()
            .unwrap()
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // NOTE: might need more depth textures to match swap chain frames in flight.
        let view_depth = self
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        (view_color, view_depth)
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.color_surface
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
