use crate::{
    render::pipeline::*, scene::scene::SceneInternal, viewport::viewport::ViewportInternal,
};

pub struct Renderer {
    device: wgpu::Device,
    instance: wgpu::Instance,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,
    mesh_render_pipeline: wgpu::RenderPipeline,
    line_strip_render_pipeline: wgpu::RenderPipeline,
    // This lives in renderer because it is needed for pipeline creation
    viewport_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer {
    pub async fn new() -> Renderer {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/mesh_shader.wgsl").into()),
        });
        let line_strip_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/line_strip_shader.wgsl").into()),
        });

        let viewport_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("scene bind group"),
            });

        let mesh_render_pipeline = create_render_pipeline(
            &device,
            &[&viewport_bind_group_layout],
            &mesh_shader,
            PipelinePrimitive::Mesh,
            1u32,
        );
        let line_strip_render_pipeline = create_render_pipeline(
            &device,
            &[&viewport_bind_group_layout],
            &line_strip_shader,
            PipelinePrimitive::LineStrip,
            1u32,
        );

        Renderer {
            device,
            queue,
            adapter,
            instance,
            mesh_render_pipeline,
            line_strip_render_pipeline,
            viewport_bind_group_layout,
        }
    }

    pub fn get_instance(&self) -> &wgpu::Instance {
        &self.instance
    }
    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }
    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }
    pub fn get_viewport_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.viewport_bind_group_layout
    }
    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn render(
        &self,
        scene: &SceneInternal,
        viewport: &ViewportInternal,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let output = viewport.get_surface().get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_bind_group(0, viewport.get_bind_group(), &[]);

            render_pass.set_pipeline(&self.mesh_render_pipeline);
            for (id, mesh) in scene.get_meshes() {
                render_pass.set_vertex_buffer(0, mesh.get_vertex_buffer().slice(..));
                render_pass
                    .set_index_buffer(mesh.get_index_buffer().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.get_index_count(), 0, 0..1);
            }
            for (id, surface) in scene.get_surfaces() {
                render_pass.set_vertex_buffer(0, surface.get_vertex_buffer().slice(..));
                render_pass.set_index_buffer(
                    surface.get_index_buffer().slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..surface.get_index_count(), 0, 0..1);
            }

            render_pass.set_pipeline(&self.line_strip_render_pipeline);
            for (id, polyline) in scene.get_polylines() {
                render_pass.set_vertex_buffer(0, polyline.get_vertex_buffer().slice(..));
                render_pass.draw(0..polyline.get_vertex_count(), 0..1);
            }
            for (id, curve) in scene.get_curves() {
                render_pass.set_vertex_buffer(0, curve.get_vertex_buffer().slice(..));
                render_pass.draw(0..curve.get_vertex_count(), 0..1);
            }
        }

        let idx = self.queue.submit(std::iter::once(encoder.finish()));
        self.device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        let output = viewport.get_surface().get_current_texture()?;
        output.present();

        Ok(())
    }
}
