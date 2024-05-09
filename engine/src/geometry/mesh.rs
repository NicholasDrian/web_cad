use wgpu::util::DeviceExt;

const DEBUG_MESH_VERTICES: &[MeshVertex] = &[
    MeshVertex {
        position: [-0.0868241, 0.49240386, 0.0],
        normal: [0.0, 0.0, 0.0],
    }, // A
    MeshVertex {
        position: [-0.49513406, 0.06958647, 0.0],
        normal: [0.5, 0.0, 0.5],
    }, // B
    MeshVertex {
        position: [-0.21918549, -0.44939706, 0.0],
        normal: [0.5, 0.0, 0.5],
    }, // C
    MeshVertex {
        position: [0.35966998, -0.3473291, 0.0],
        normal: [0.5, 0.0, 0.5],
    }, // D
    MeshVertex {
        position: [0.44147372, 0.2347359, 0.0],
        normal: [0.5, 0.0, 0.5],
    }, // E
];

const DEBUG_MESH_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

pub static MESH_VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> =
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    };

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl Mesh {
    pub fn new(device: wgpu::Device) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(DEBUG_MESH_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(DEBUG_MESH_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        Mesh {
            vertex_buffer,
            index_buffer,
        }
    }
}
