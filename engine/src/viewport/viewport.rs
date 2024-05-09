use super::camera::Camera;
use web_sys::GpuCanvasContext;
use web_sys::GpuTextureFormat;
use web_sys::HtmlCanvasElement;

pub struct Viewport {
    camera: Camera,
    canvas: HtmlCanvasElement,
}

impl Viewport {
    pub fn new(canvas: HtmlCanvasElement) -> Viewport {
        Viewport {
            camera: Camera::default(),
            canvas,
        }
    }
}
