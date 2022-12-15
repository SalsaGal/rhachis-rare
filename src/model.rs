use std::{mem::size_of, sync::Arc};

use rhachis::{graphics::BufferData, renderers::Transform, GameData};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, VertexBufferLayout,
};

use crate::material::Material;

pub struct Model {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) indices: BufferData<u16>,
    pub(crate) transforms: BufferData<Transform, [[f32; 4]; 4]>,
    pub(crate) material: Arc<Material>,
}

impl Model {
    pub fn new(
        data: &GameData,
        vertices: Vec<TextureVertex>,
        indices: Vec<u16>,
        material: Arc<Material>,
        transforms: Vec<Transform>,
    ) -> Self {
        let vertex_buffer = data
            .graphics
            .device
            .create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let indices = BufferData::new(data, indices, wgpu::BufferUsages::INDEX);
        let transforms = BufferData::new(data, transforms, wgpu::BufferUsages::VERTEX);

        Self {
            vertex_buffer,
            indices,
            material,
            transforms,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    pub pos: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl TextureVertex {
    pub(crate) fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                },
            ],
        }
    }
}
