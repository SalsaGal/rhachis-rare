use std::mem::size_of;

use rhachis::{graphics::BufferData, GameData};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, VertexBufferLayout,
};

pub struct Model {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) indices: BufferData<u16>,
}

impl Model {
    pub fn new(data: &GameData, vertices: &[TextureVertex], indices: &[u16]) -> Self {
        let vertex_buffer = data
            .graphics
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let indices = BufferData::new(data, indices.to_vec(), wgpu::BufferUsages::INDEX);

        Self {
            vertex_buffer,
            indices,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    pub pos: [f32; 3],
}

impl TextureVertex {
    pub(crate) fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        }
    }
}
