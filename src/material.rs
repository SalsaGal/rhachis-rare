use std::sync::{Arc, Weak};

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
}

pub struct MaterialManager {
    pub demand_mats: IdMap<Weak<Material>>,
    pub static_mats: IdMap<Arc<Material>>,
    pub error_mat: Material,
}

impl MaterialManager {
    pub(crate) fn new(data: &GameData) -> Self {
        Self {
            demand_mats: IdMap::new(),
            static_mats: IdMap::new(),
            error_mat: Material::error(data),
        }
    }
}

pub enum MaterialError {
    ImageError(ImageError),
}
