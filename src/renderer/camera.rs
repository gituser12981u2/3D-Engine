use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    position: Vec3,
    orientation: Quat,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(position: Vec3, fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        println!("Camera aspect ratio: {aspect_ratio}");
        Camera {
            position,
            orientation: Quat::IDENTITY,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }

    // pub fn set_position(&mut self, position: Vec3) {
    //     self.position = position;
    // }

    // pub fn set_orientation(&mut self, orientation: Quat) {
    //     self.orientation = orientation;
    // }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::from_quat(self.orientation).inverse() * Mat4::from_translation(-self.position)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    // pub fn move_camera(&mut self, direction: Vec3) {
    //     self.position += direction;
    // }

    // pub fn rotate_camera(&mut self, pitch: f32, yaw: f32) {
    //     let pitch_rot = Quat::from_rotation_x(pitch);
    //     let yaw_rot = Quat::from_rotation_y(yaw);
    //     self.orientation = yaw_rot * pitch_rot * self.orientation;
    // }
}
