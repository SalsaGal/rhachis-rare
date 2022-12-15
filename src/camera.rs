use glam::{Mat4, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub ty: CameraType,
    pub fov: f32,
    pub aspect: f32,
}

impl From<Camera> for [[f32; 4]; 4] {
    fn from(cam: Camera) -> Self {
        let view = match cam.ty {
            CameraType::LookAt(center) => Mat4::look_at_rh(cam.pos, center, Vec3::Y),
            CameraType::LookTo(dir) => Mat4::look_to_rh(cam.pos, dir, Vec3::Y),
        };
        let proj = Mat4::perspective_infinite_rh(cam.fov, cam.aspect, f32::EPSILON);
        (proj * view).to_cols_array_2d()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CameraType {
    LookAt(Vec3),
    LookTo(Vec3),
}
