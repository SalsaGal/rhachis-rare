use std::sync::{Weak, Arc};

use rhachis::{renderers::Texture, IdMap};

pub struct Material {
    pub color: Texture,
}

pub struct MaterialManager {
    pub demand_mats: IdMap<Weak<Material>>,
    pub static_mats: IdMap<Arc<Material>>,
}

impl MaterialManager {
    pub(crate) fn new() -> Self {
        Self {
            demand_mats: IdMap::new(),
            static_mats: IdMap::new(),
        }
    }
}
