use wasm_bindgen::prelude::*;

use crate::{
    instance::Handle,
    utils::get_instance_mut,
    viewport::camera::{CameraDescriptor, CameraType},
};

#[wasm_bindgen]
pub struct Viewport {
    instance_handle: Handle,
    viewport_handle: Handle,
}

#[wasm_bindgen]
impl Viewport {
    pub fn new(instance_handle: Handle, viewport_handle: Handle) -> Viewport {
        Viewport {
            instance_handle,
            viewport_handle,
        }
    }
    pub fn get_handle(&self) -> Handle {
        self.viewport_handle
    }

    #[wasm_bindgen]
    pub fn set_camera_params(
        &mut self,
        position: &[f32],
        focal_point: &[f32],
        fovy: f32,
        aspect: f32,
        near_dist: f32,
        far_dist: f32,
        camera_type: CameraType,
    ) {
        get_instance_mut!(&self.instance_handle)
            .get_viewport_mut(self.viewport_handle)
            .get_camera_mut()
            .update_params(CameraDescriptor {
                position: position.into(),
                focal_point: focal_point.into(),
                fovy,
                aspect,
                near_dist,
                far_dist,
                camera_type,
            });
        get_instance_mut!(&self.instance_handle)
            .get_viewport_mut(self.viewport_handle)
            .update_bind_group();
    }
}
