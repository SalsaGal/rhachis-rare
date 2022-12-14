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
