use rhachis::{*, graphics::EmptyRenderer};

#[rhachis::run]
struct Simple {
    renderer: EmptyRenderer,
}

impl Game for Simple {
    fn init(_: &GameData) -> Self {
        Self { renderer: EmptyRenderer }
    }

    fn get_renderer(&mut self) -> &mut dyn graphics::Renderer {
        &mut self.renderer
    }
}
