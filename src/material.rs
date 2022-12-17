use image::ImageError;
use rhachis::{renderers::Texture, GameData};

pub struct Material {
    pub color: Texture,
}

impl Material {
    pub fn error(data: &GameData) -> Material {
        let error_image = image::load_from_memory(include_bytes!("error.png")).unwrap();

        Self {
            color: Texture::from_image(
                data,
                error_image.as_rgba8().unwrap(),
                &rhachis::graphics::SamplerType::Nearest,
            )
            .unwrap(),
        }
    }

    pub fn bind_group_layout(data: &GameData) -> wgpu::BindGroupLayout {
        data.graphics
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            })
    }
}

pub enum MaterialError {
    ImageError(ImageError),
}
