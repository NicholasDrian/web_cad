use crate::render::renderer::Renderer;

use super::camera::Camera;
use web_sys::GpuCanvasContext;
use web_sys::GpuTextureFormat;
use web_sys::HtmlCanvasElement;

pub struct Viewport {
    camera: Camera,
    canvas: HtmlCanvasElement,
    surface: wgpu::Surface<'static>,
}

impl Viewport {
    pub fn new(canvas: HtmlCanvasElement, renderer: &Renderer) -> Viewport {
        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.clone());
        let surface = renderer
            .get_instance()
            .create_surface(surface_target)
            .unwrap();
        let config = surface
            .get_default_config(&renderer.get_adapter(), canvas.width(), canvas.height())
            .unwrap();
        surface.configure(renderer.get_device(), &config);

        Viewport {
            camera: Camera::default(),
            canvas,
            surface,
        }
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.surface
    }
}
