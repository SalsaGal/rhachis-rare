use std::sync::Arc;

use renderer::{
    model::{Model, TextureVertex},
    Renderer, material::Material,
};
use rhachis::{renderers::Transform, *};

#[rhachis::run]
struct Simple {
    renderer: Renderer,
}

impl Game for Simple {
    fn init(data: &GameData) -> Self {
        let mut renderer = Renderer::new(data);
        let material = Arc::new(Material::error(data));
        renderer.models.push(Model::new(
            data,
            vec![
                TextureVertex {
                    pos: [0.0, 0.0, 0.0],
                    tex_coords: [0.0, 1.0],
                },
                TextureVertex {
                    pos: [1.0, 0.0, 0.0],
                    tex_coords: [1.0, 1.0],
                },
                TextureVertex {
                    pos: [0.0, 1.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
            ],
            vec![0, 1, 2],
            material,
            vec![Transform::scale((1.0, 0.5, 1.0))],
        ));

        Self { renderer }
    }

    fn get_renderer(&mut self) -> &mut dyn graphics::Renderer {
        &mut self.renderer
    }
}
