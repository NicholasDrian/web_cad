use std::{rc::Rc, time::Instant};

use crate::{
    math::linear_algebra::{mat4::Mat4, vec3::Vec3},
    render::renderer::Renderer,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum CameraType {
    /// This is a first person shooter style camera.
    /// Rotation is around the cameras position
    FPS,
    /// This is a CAD style camera.
    /// Rotation is around the focal point
    CAD,
}

pub struct CameraDescriptor {
    pub position: Vec3,
    pub focal_point: Vec3,
    pub fovy: f32,
    pub aspect: f32,
    pub near_dist: f32,
    pub far_dist: f32,
    pub camera_type: CameraType,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            focal_point: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            fovy: 1.0,
            aspect: 1.0,
            near_dist: 0.01,
            far_dist: 10000.0,
            camera_type: CameraType::CAD,
        }
    }
}

pub struct Camera {
    position: Vec3,
    focal_point: Vec3,
    up: Vec3,
    /// Vertical field of view
    fovy: f32,
    aspect: f32,
    /// Closest distance that is rendered
    near_dist: f32,
    /// Farthest distance that is rendered
    far_dist: f32,
    /// Set to none when out of date
    view_proj: Mat4,
    view_proj_buffer: wgpu::Buffer,
    camera_type: CameraType,
    last_frame_time: Option<Instant>,
    renderer: Rc<Renderer>,
}

//TODO: toggle for auto motion... eventually
impl Camera {
    pub fn new(params: CameraDescriptor, renderer: Rc<Renderer>) -> Self {
        let forward = Vec3::subtract(&params.focal_point, &params.position);
        let mut up = if forward.x == 0.0 && forward.z == 0.0 {
            // Camera is vertical
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        };
        let right = Vec3::cross(&forward, &up);
        up = *Vec3::cross(&right, &forward).normalize();

        let mut res = Camera {
            position: params.position,
            focal_point: params.focal_point,
            fovy: params.fovy,
            aspect: params.aspect,
            near_dist: params.near_dist,
            far_dist: params.far_dist,
            up,
            camera_type: params.camera_type,
            view_proj: Mat4::identity(),
            view_proj_buffer: renderer
                .get_device()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("camera view proj buffer"),
                    size: std::mem::size_of::<Mat4>() as u64,
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                    mapped_at_creation: false,
                }),
            last_frame_time: None,
            renderer: renderer.clone(),
        };
        res.update_view_proj();
        res
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.update_view_proj();
    }

    pub fn update_params(&mut self, params: CameraDescriptor) {
        let forward = Vec3::subtract(&params.focal_point, &params.position);
        let mut up = if forward.x == 0.0 && forward.z == 0.0 {
            // Camera is vertical
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        };
        let right = Vec3::cross(&forward, &up);
        up = *Vec3::cross(&right, &forward).normalize();
        self.position = params.position;
        self.focal_point = params.focal_point;
        self.fovy = params.fovy;
        self.aspect = params.aspect;
        self.near_dist = params.near_dist;
        self.far_dist = params.far_dist;
        self.up = up;
        self.camera_type = params.camera_type;
        self.view_proj = Mat4::identity();
        self.view_proj_buffer = self
            .renderer
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("camera view proj buffer"),
                size: std::mem::size_of::<Mat4>() as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                mapped_at_creation: false,
            });
        self.update_view_proj();
    }

    pub fn get_view_proj(&self) -> Mat4 {
        self.view_proj
    }

    pub fn set_camera_type(&mut self, camera_type: CameraType) {
        self.camera_type = camera_type;
    }

    pub fn turn_up(&mut self, theta: f32) -> &mut Self {
        match self.camera_type {
            CameraType::CAD => {}
            CameraType::FPS => {}
        }
        self.update_view_proj();
        self
    }
    pub fn look_right(&mut self, theta: f32) -> &mut Self {
        match self.camera_type {
            CameraType::CAD => {
                todo!();
            }
            CameraType::FPS => {
                todo!();
            }
        }
        self.update_view_proj();
        self
    }

    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        todo!();
        self.update_view_proj();
        self
    }

    pub fn zoom(&mut self, scale: f32) -> &mut Self {
        match self.camera_type {
            CameraType::CAD => {
                // Move camera closer to focal point
                todo!();
            }
            CameraType::FPS => {
                // Move focal point closer to camera
                todo!();
            }
        }
        self.update_view_proj();
        self
    }

    fn update_view_proj(&mut self) {
        let view = Mat4::look_at(&self.position, &self.focal_point, &self.up);
        let proj = Mat4::perspective(self.fovy, self.aspect, self.near_dist, self.far_dist);
        self.view_proj = Mat4::multiply(&proj, &view);
        self.renderer.get_queue().write_buffer(
            &self.view_proj_buffer,
            0,
            bytemuck::cast_slice(&[self.view_proj]),
        );
    }

    pub fn get_view_proj_buffer(&self) -> &wgpu::Buffer {
        &self.view_proj_buffer
    }
}
