use crate::{render::renderer::Renderer, utils::create_compute_pipeline};

pub fn create_radix_sort_resources(
    renderer: &Renderer,
) -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
    let device = renderer.get_device();

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("radix sort"),
        entries: &[
            // Params
            crate::utils::compute_uniform_bind_group_layout_entry(0),
            // Keys
            crate::utils::compute_buffer_bind_group_layout_entry(1, false),
            // Values
            crate::utils::compute_buffer_bind_group_layout_entry(2, false),
        ],
    });

    let pipeline = create_compute_pipeline(
        device,
        "radix sort",
        include_str!("radix_sort.wgsl"),
        &bind_group_layout,
        "radix sort",
    );
    (bind_group_layout, pipeline)
}
