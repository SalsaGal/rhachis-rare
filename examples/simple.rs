use std::{f32::consts::TAU, sync::Arc};

use glam::Vec3;
use rare::{
    camera::{Camera, CameraType},
    material::Material,
    model::{Model, TextureVertex},
    Renderer,
};
use rhachis::{
    input::{InputState, Key},
    renderers::{Texture, Transform},
    *,
};

#[rhachis::run(rhachis::GameInit::from(Renderer::FEATURES))]
struct Simple {
    renderer: Renderer,
}

impl Game for Simple {
    fn init(data: &GameData) -> Self {
        let mut renderer = Renderer::new(data);
        renderer.camera[0] = Camera {
            pos: Vec3::Z,
            ty: CameraType::LookAt(Vec3::ZERO),
            fov: TAU / 4.0,
            aspect: data.get_window_size().x as f32 / data.get_window_size().y as f32,
        };
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
                    normals: [0.0, 0.0, 1.0],
                },
                TextureVertex {
                    pos: [1.0, 0.0, 0.0],
                    tex_coords: [1.0, 1.0],
                    normals: [0.0, 0.0, 1.0],
                },
                TextureVertex {
                    pos: [0.0, 1.0, 0.0],
                    tex_coords: [0.0, 0.0],
                    normals: [0.0, 0.0, 1.0],
                },
            ],
            vec![0, 1, 2],
            material,
            vec![Transform::translation([1.0, 0.0, 0.0])],
        ));
        renderer.load_gltf(data, "examples/monkey.gltf", 0);

        Self { renderer }
    }

    fn update(&mut self, data: &GameData) {
        let x = f32::sin(data.start_time.elapsed().as_secs_f32()).abs();
        self.renderer.models[0].transforms[0].scale.y = x;

        if data.input.is_key(Key::Right, InputState::Down) {
            self.renderer.camera[0].pos.x += 1.0 * data.delta_time.as_secs_f32();
        } else if data.input.is_key(Key::Left, InputState::Down) {
            self.renderer.camera[0].pos.x -= 1.0 * data.delta_time.as_secs_f32();
        }
        if data.input.is_key(Key::Space, InputState::Pressed) {
            self.renderer.camera[0].ty = match self.renderer.camera[0].ty {
                CameraType::LookAt(..) => CameraType::LookTo(Vec3::NEG_Z),
                CameraType::LookTo(..) => CameraType::LookAt(Vec3::ZERO),
            };
        }
        self.renderer.camera.update(data);
    }

    fn get_renderer(&mut self) -> &mut dyn graphics::Renderer {
        &mut self.renderer
    }
}
