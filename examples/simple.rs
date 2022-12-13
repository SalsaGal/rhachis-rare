use renderer::{Renderer, model::{Model, TextureVertex}};
use rhachis::*;

#[rhachis::run]
struct Simple {
    renderer: Renderer,
}

impl Game for Simple {
    fn init(data: &GameData) -> Self {
        let mut renderer = Renderer::new(data);
        renderer.models.push(Model::new(
            data,
            &[
                TextureVertex {
                    pos: [0.0, 0.0, 0.0],
                },
                TextureVertex {
                    pos: [1.0, 0.0, 0.0],
                },
                TextureVertex {
                    pos: [0.0, 1.0, 0.0],
                },
            ],
            &[
                0, 1, 2
            ]
        ));

        Self {
            renderer
        }
    }

    fn get_renderer(&mut self) -> &mut dyn graphics::Renderer {
        &mut self.renderer
    }
}
