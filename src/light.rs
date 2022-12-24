use glam::Vec3;
use rhachis::graphics::{Bindable, BufferCompatible};
use wgpu::Color;

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub pos: Vec3,
    pub color: Color,
}

impl BufferCompatible for Light {
    type PodFormat = LightUniform;
    fn into_pod(self) -> Self::PodFormat {
        self.into()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

impl Bindable for LightUniform {
    fn bind_group_layout(data: &rhachis::GameData) -> wgpu::BindGroupLayout {
        data.graphics
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            })
    }
}

impl From<Light> for LightUniform {
    fn from(value: Light) -> Self {
        LightUniform {
            pos: value.pos.extend(0.0).to_array(),
            color: [
                value.color.r as f32,
                value.color.g as f32,
                value.color.b as f32,
                0.0,
            ],
        }
    }
}
