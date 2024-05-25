pub mod camera;
pub mod viewport_interface;
use std::rc::Rc;

use crate::math::linear_algebra::mat4::Mat4;
use crate::render::renderer::Renderer;

use camera::{Camera, CameraDescriptor};
use web_sys::HtmlCanvasElement;

pub struct ViewportInternal {
    renderer: Rc<Renderer>,
    camera: Camera,
    canvas: HtmlCanvasElement,
    color_surface: wgpu::Surface<'static>,
    bind_group: wgpu::BindGroup,
    depth_texture: wgpu::Texture,
    color_texture: wgpu::Texture,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewportUniforms {
    view_proj: Mat4,
}

impl ViewportInternal {
    pub fn new(
        canvas: HtmlCanvasElement,
        renderer: Rc<Renderer>,
        sample_count: u32,
    ) -> ViewportInternal {
        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.clone());
        let surface = renderer
            .get_instance()
            .create_surface(surface_target)
            .unwrap();
        let config = surface
            .get_default_config(renderer.get_adapter(), canvas.width(), canvas.height())
            .unwrap();

        surface.configure(renderer.get_device(), &config);

        let mut camera = Camera::new(CameraDescriptor::default(), renderer.clone());
        camera.set_aspect(canvas.width() as f32 / canvas.height() as f32);

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
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24Plus,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
        let color_texture = renderer
            .get_device()
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("color texture"),
                size: wgpu::Extent3d {
                    width: canvas.width(),
                    height: canvas.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count,
                dimension: wgpu::TextureDimension::D2,
                format: config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

        ViewportInternal {
            renderer,
            camera,
            canvas,
            color_surface: surface,
            bind_group,
            depth_texture,
            color_texture,
        }
    }
    pub fn update_bind_group(&mut self) -> &mut Self {
        self.bind_group =
            self.renderer
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("scene bind group"),
                    layout: self.renderer.get_viewport_bind_group_layout(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.camera.get_view_proj_buffer().as_entire_binding(),
                    }],
                });
        self
    }

    pub fn get_views(&self) -> (wgpu::TextureView, wgpu::TextureView, wgpu::TextureView) {
        // NOTE: might need more depth textures to match swap chain frames in flight.
        // NOTE: maybe I can make these views before hand
        let view_color = self
            .color_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let view_depth = self
            .depth_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let resolve_target = self
            .color_surface
            .get_current_texture()
            .unwrap()
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        (view_color, view_depth, resolve_target)
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.color_surface
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}
