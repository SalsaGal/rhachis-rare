use glam::{Mat4, Vec3};
use rhachis::{graphics::BufferCompatible, GameData};

#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    pub pos: Vec3,
    pub ty: CameraType,
    pub fov: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn update_aspect(&mut self, data: &GameData) {
        self.aspect = data.get_window_size().x as f32 / data.get_window_size().y as f32;
    }
}

impl BufferCompatible for Camera {
    type PodFormat = [[f32; 4]; 4];
    fn into_pod(self) -> Self::PodFormat {
        self.into()
    }
}

impl From<Camera> for [[f32; 4]; 4] {
    fn from(cam: Camera) -> Self {
        let view = match cam.ty {
            CameraType::LookAt(center) => Mat4::look_at_rh(cam.pos, center, Vec3::Y),
            CameraType::LookTo(dir) => Mat4::look_to_rh(cam.pos, dir, Vec3::Y),
        };
        let proj = Mat4::perspective_infinite_rh(cam.fov, cam.aspect, 0.1);
        (proj * view).to_cols_array_2d()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CameraType {
    LookAt(Vec3),
    LookTo(Vec3),
}

impl Default for CameraType {
    fn default() -> Self {
        Self::LookTo(Vec3::NEG_Z)
    }
}
