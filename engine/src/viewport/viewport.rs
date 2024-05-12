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
    surface: wgpu::Surface<'static>,
    bind_group: wgpu::BindGroup,
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
            .get_default_config(&renderer.get_adapter(), canvas.width(), canvas.height())
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

        ViewportInternal {
            camera,
            canvas,
            surface,
            bind_group,
        }
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
