use rhachis::GameData;

pub struct Renderer {
}

impl Renderer {
    pub fn new(data: &GameData) -> Self {
        Self {}
    }
}

impl rhachis::graphics::Renderer for Renderer {}
