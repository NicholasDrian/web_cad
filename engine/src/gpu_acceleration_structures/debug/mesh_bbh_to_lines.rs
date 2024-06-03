use std::rc::Rc;

use crate::{
    geometry::lines::Lines, gpu_acceleration_structures::mesh_bbh::MeshBBH,
    render::renderer::Renderer, utils::create_compute_pipeline,
};

// Rebuilding pipeline and such every call.
// Expensive, but this is only for debug so its chill
pub fn mesh_bbh_to_lines(renderer: Rc<Renderer>, mesh_bbh: &MeshBBH) -> Lines {
    let node_count = mesh_bbh.get_node_count();
    let tree_buffer = mesh_bbh.get_tree();

    let device = renderer.get_device();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("build next level"),
        entries: &[
            // tree
            crate::utils::compute_buffer_bind_group_layout_entry(0, true),
            // vertex buffer
            crate::utils::compute_buffer_bind_group_layout_entry(1, false),
            // index buffer
            crate::utils::compute_buffer_bind_group_layout_entry(2, false),
        ],
    });
    let pipeline = create_compute_pipeline(
        device,
        "mesh bb to lines",
        include_str!("mesh_bbh_to_lines.wgsl"),
        &bind_group_layout,
        "main",
    );

    const VERTICES_PER_BB: u32 = 8;
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("vertex buffer"),
        size: (node_count * VERTICES_PER_BB * 16) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
        mapped_at_creation: false,
    });
    const INDICES_PER_BB: u32 = 24;
    let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("index buffer"),
        size: (node_count * INDICES_PER_BB * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::INDEX,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("mesh bbh to lines"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: tree_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: vertex_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: index_buffer.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("mesh bbh to lines"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("mesh bbh to lines"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(node_count, 1, 1);
    }

    let idx = renderer.get_queue().submit([encoder.finish()]);
    device.poll(wgpu::Maintain::WaitForSubmissionIndex(idx));

    Lines::from_buffers(
        renderer,
        vertex_buffer,
        index_buffer,
        node_count * INDICES_PER_BB,
    )
}
