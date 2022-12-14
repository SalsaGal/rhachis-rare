use std::sync::{Weak, Arc};

use image::ImageError;
use rhachis::{renderers::Texture, GameData, IdMap};

pub struct Material {
    pub color: Texture,
}

impl Material {
    pub fn error(data: &GameData) -> Material {
        let error_image = image::load_from_memory(include_bytes!("error.png")).unwrap();

        Self {
            color: Texture::new(
                data,
                error_image.as_rgba8().unwrap(),
                &rhachis::graphics::SamplerType::Nearest,
            )
            .unwrap(),
        }
    }

    pub fn bind_group_layout(data: &GameData) -> wgpu::BindGroupLayout {
        data.graphics.device
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

pub struct MaterialManager {
    pub droppable_mats: IdMap<Weak<Material>>,
    pub hold_mats: IdMap<Arc<Material>>,
    pub error_mat: Arc<Material>,
}

impl MaterialManager {
    pub fn new(data: &GameData) -> Self {
        Self {
            droppable_mats: IdMap::new(),
            hold_mats: IdMap::new(),
            error_mat: Arc::new(Material::error(data)),
        }
    }
}
