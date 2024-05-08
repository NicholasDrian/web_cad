use super::camera::Camera;
use web_sys::HtmlCanvasElement;

pub struct Viewport {
    camera: Camera,
}

impl Viewport {
    pub fn new(canvas: HtmlCanvasElement) -> Viewport {
        Viewport {
            camera: Camera::default(),
        }
    }
}
