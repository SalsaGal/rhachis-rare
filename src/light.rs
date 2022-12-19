use glam::Vec3;
use rhachis::graphics::Bindable;

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub pos: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub pos: [f32; 3],
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
                        ty: wgpu::BufferBindingType::Uniform,
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
            pos: value.pos.to_array(),
        }
    }
}
