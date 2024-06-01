use crate::{render::pipeline::*, scene::SceneInternal, viewport::ViewportInternal};

pub struct Renderer {
    device: wgpu::Device,
    instance: wgpu::Instance,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,
    mesh_render_pipeline: wgpu::RenderPipeline,
    line_strip_render_pipeline: wgpu::RenderPipeline,
    lines_render_pipeline: wgpu::RenderPipeline,
    viewport_bind_group_layout: wgpu::BindGroupLayout,
    geometry_bind_group_layout: wgpu::BindGroupLayout,
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
                label: Some("viewport bind group layout"),
            });
        let geometry_bind_group_layout =
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
                label: Some("geometry bind group layout"),
            });

        let mesh_render_pipeline = create_render_pipeline(
            &device,
            &[&viewport_bind_group_layout, &geometry_bind_group_layout],
            &mesh_shader,
            PipelinePrimitive::Mesh,
            4u32,
        );
        let line_strip_render_pipeline = create_render_pipeline(
            &device,
            &[&viewport_bind_group_layout, &geometry_bind_group_layout],
            &line_strip_shader,
            PipelinePrimitive::LineStrip,
            4u32,
        );
        let lines_render_pipeline = create_render_pipeline(
            &device,
            &[&viewport_bind_group_layout, &geometry_bind_group_layout],
            &line_strip_shader,
            PipelinePrimitive::Lines,
            4u32,
        );

        Renderer {
            device,
            queue,
            adapter,
            instance,
            mesh_render_pipeline,
            line_strip_render_pipeline,
            lines_render_pipeline,
            viewport_bind_group_layout,
            geometry_bind_group_layout,
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
    pub fn get_geometry_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.geometry_bind_group_layout
    }
    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn render(&self, scene: &SceneInternal, viewport: &ViewportInternal) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let (color_view, depth_view, resolve_target) = viewport.get_views();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color_view,
                    resolve_target: Some(&resolve_target),
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations::<f32> {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_bind_group(0, viewport.get_bind_group(), &[]);

            render_pass.set_pipeline(&self.mesh_render_pipeline);
            for mesh in scene.get_meshes().values() {
                render_pass.set_bind_group(1, mesh.get_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, mesh.get_vertex_buffer().slice(..));
                render_pass
                    .set_index_buffer(mesh.get_index_buffer().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..mesh.get_index_count(), 0, 0..1);
            }
            for surface in scene.get_surfaces().values() {
                render_pass.set_bind_group(1, surface.get_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, surface.get_vertex_buffer().slice(..));
                render_pass.set_index_buffer(
                    surface.get_index_buffer().slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..surface.get_index_count(), 0, 0..1);
            }

            render_pass.set_pipeline(&self.line_strip_render_pipeline);
            for curve in scene.get_curves().values() {
                render_pass.set_bind_group(1, curve.get_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, curve.get_vertex_buffer().slice(..));
                render_pass.draw(0..curve.get_vertex_count(), 0..1);
            }
            for polyline in scene.get_polylines().values() {
                render_pass.set_bind_group(1, polyline.get_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, polyline.get_vertex_buffer().slice(..));
                render_pass.draw(0..polyline.get_vertex_count(), 0..1);
            }
            render_pass.set_pipeline(&self.lines_render_pipeline);
            for lines in scene.get_lines().values() {
                render_pass.set_bind_group(1, lines.get_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, lines.get_vertex_buffer().slice(..));
                render_pass.set_index_buffer(
                    lines.get_index_buffer().slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..lines.get_index_count(), 0, 0..1);
            }
        }

        let idx = self.queue.submit(std::iter::once(encoder.finish()));
        self.device
            .poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

        let output = viewport.get_surface().get_current_texture().unwrap();
        output.present();
    }
}
