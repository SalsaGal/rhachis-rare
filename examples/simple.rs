use std::sync::Arc;

use renderer::{
    material::Material,
    model::{Model, TextureVertex},
    Renderer,
};
use rhachis::{
    renderers::{Texture, Transform},
    *,
};

#[rhachis::run]
struct Simple {
    renderer: Renderer,
}

impl Game for Simple {
    fn init(data: &GameData) -> Self {
        let mut renderer = Renderer::new(data);
        let material = Arc::new(Material {
            color: Texture::from_path(data, "examples/test.png", &graphics::SamplerType::Linear)
                .unwrap(),
        });
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
            vec![Transform::default()],
        ));

        Self { renderer }
    }

    fn update(&mut self, data: &GameData) {
        let x = f32::sin(data.start_time.elapsed().as_secs_f32()).abs();
        self.renderer.models[0].transforms[0] = Transform::scale((1.0, x, 1.0));
        dbg!(x);
    }

    fn get_renderer(&mut self) -> &mut dyn graphics::Renderer {
        &mut self.renderer
    }
}
