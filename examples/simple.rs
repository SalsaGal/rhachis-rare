use renderer::{
    model::{Model, TextureVertex},
    Renderer,
};
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
            renderer.materials.error_mat.clone(),
        ));

        Self { renderer }
    }

    fn get_renderer(&mut self) -> &mut dyn graphics::Renderer {
        &mut self.renderer
    }
}
