use glam::{Mat4, Quat, Vec3};

pub struct Camera {
    position: Vec3,
    orientation: Quat,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    movement_speed: f32,
    mouse_sensitivity: f32,
}

impl Camera {
    pub fn new(position: Vec3, fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Camera {
            position,
            orientation: Quat::IDENTITY,
            fov,
            aspect_ratio,
            near,
            far,
            movement_speed: 0.5,
            mouse_sensitivity: 0.001,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let forward = self.orientation * -Vec3::Z;
        let up = self.orientation * Vec3::Y;
        Mat4::look_at_rh(self.position, self.position + forward, up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        let forward = self.orientation * -Vec3::Z;
        let right = self.orientation * Vec3::X;
        let up = Vec3::Y; // World up vector

        match direction {
            CameraMovement::Forward => self.position += forward * velocity,
            CameraMovement::Backward => self.position -= forward * velocity,
            CameraMovement::Left => self.position -= right * velocity,
            CameraMovement::Right => self.position += right * velocity,
            CameraMovement::Up => self.position += up * velocity,
            CameraMovement::Down => self.position -= up * velocity,
        }
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        let pitch_rotation = Quat::from_axis_angle(Vec3::X, y_offset);
        let yaw_rotation = Quat::from_axis_angle(Vec3::Y, -x_offset);

        // Apply yaw globally, pitch locally
        self.orientation = yaw_rotation * self.orientation * pitch_rotation;
        self.orientation = self.orientation.normalize();

        println!(
            "Camera orientation after mouse movement: {:?}",
            self.orientation
        );
    }

    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.fov -= y_offset;
        self.fov = self.fov.clamp(1.0, 90.0);
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use crate::renderer::{camera::CameraMovement, Camera};
    use glam::{Mat3, Mat4, Vec3};
    use objc::rc::autoreleasepool;
    use std::panic;

    fn log_test_start(test_name: &str) {
        println!("Starting test: {test_name}")
    }

    fn log_test_end(test_name: &str) {
        println!("Finished test: {test_name}")
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3, epsilon: f32) -> bool {
        (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon
    }

    fn mat4_approx_orthogonal(mat: Mat4, epsilon: f32) -> Result<(), String> {
        // Extract the upper-left 3x3 submatrix (rotation part)
        let rotation = Mat3::from_cols(
            mat.x_axis.truncate(),
            mat.y_axis.truncate(),
            mat.z_axis.truncate(),
        );

        let product = rotation * rotation.transpose();
        let identity = Mat3::IDENTITY;

        let mut max_diff = 0.0;
        let mut max_diff_pos = (0, 0);

        for row in 0..3 {
            for col in 0..3 {
                let diff = (product.col(col)[row] - identity.col(col)[row]).abs();
                if diff > max_diff {
                    max_diff = diff;
                    max_diff_pos = (row, col);
                }
            }
        }

        if max_diff > epsilon {
            Err(format!("Rotation part of the matrix is not approximately orthogonal. Max difference of {} at position {:?}", max_diff, max_diff_pos))
        } else {
            Ok(())
        }
    }

    fn f32_approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_view_matrix() {
        log_test_start("test_view_matrix");
        autoreleasepool(|| {
            let camera_pos = Vec3::new(0.0, 0.0, 5.0);
            let camera = Camera::new(camera_pos, 45.0, 1.0, 0.1, 100.0);
            let view_matrix = camera.get_view_matrix();
            let epsilon = 1e-6;

            // Check orthogonality
            assert!(
                mat4_approx_orthogonal(view_matrix, epsilon).is_ok(),
                "View matrix rotation part is not approximately orthogonal: {}",
                mat4_approx_orthogonal(view_matrix, epsilon)
                    .err()
                    .unwrap_or_default()
            );

            // Check that the camera position is correctly transformed
            let transformed_camera_pos = view_matrix.transform_point3(camera_pos);
            assert!(
                vec3_approx_eq(transformed_camera_pos, Vec3::ZERO, epsilon),
                "Camera position not correctly transformed. Expected: {:?}, Got: {:?}",
                Vec3::ZERO,
                transformed_camera_pos
            );

            // Check that forward direction is along negative z-axis in view space
            let world_forward = Vec3::new(0.0, 0.0, -1.0);
            let view_forward = view_matrix.transform_vector3(world_forward).normalize();
            assert!(
                vec3_approx_eq(view_forward, Vec3::NEG_Z, epsilon),
                "Forward direction not correctly transformed. Expected: {:?}, Got: {:?}",
                Vec3::NEG_Z,
                view_forward
            );

            // Test that up direction is along positive y-axis in view space
            let world_up = Vec3::Y;
            let view_up = view_matrix.transform_vector3(world_up).normalize();
            assert!(
                vec3_approx_eq(view_up, Vec3::Y, epsilon),
                "Up direction not correctly transformed. Expected: {:?}, Got: {:?}",
                Vec3::Y,
                view_up
            );

            println!("View Matrix:\n{:?}", view_matrix);
        });
        log_test_end("test_view_matrix");
    }

    #[test]
    fn test_projection_matrix() {
        log_test_start("test_projection_matrix");
        autoreleasepool(|| {
            let fov = 45.0f32;
            let aspect_ratio = 16.0 / 9.0;
            let near = 0.1;
            let far = 100.0;
            let camera = Camera::new(Vec3::ZERO, fov, aspect_ratio, near, far);
            let proj_matrix = camera.get_projection_matrix();
            let epsilon = 1e-6;

            // Check that Z-near and Z-far affect the z-axis scaling
            let z_scale = proj_matrix.z_axis.z;
            assert!(
                z_scale < -1.0 && z_scale > -2.0,
                "Z-scaling is out of expected range"
            );

            // Check that perspective division is set up correctly
            assert!(
                f32_approx_eq(proj_matrix.z_axis.w, -1.0, epsilon),
                "Perspective division factor is incorrect"
            );

            // Check that FOV affects vertical scaling
            let expected_y_scale = 1.0 / (0.5 * fov).to_radians().tan();
            assert!(
                f32_approx_eq(proj_matrix.y_axis.y, expected_y_scale, epsilon),
                "Vertical scaling does not match expected FOV"
            );

            // Check that the matrix is indeed a projection matrix
            assert!(
                proj_matrix.w_axis.x == 0.0
                    && proj_matrix.w_axis.y == 0.0
                    && proj_matrix.w_axis.w == 0.0,
                "Matrix does not have the expected structure of a projection matrix"
            );

            println!("Projection Matrix:\n{:?}", proj_matrix);
        });
        log_test_end("test_projection_matrix");
    }

    #[test]
    fn test_camera_movement() {
        log_test_start("test_camera_movement");
        autoreleasepool(|| {
            let result = panic::catch_unwind(|| {
                let position = Vec3::ZERO;
                let fov = 45.0;
                let aspect_ratio = 1.0;
                let near = 0.1;
                let far = 100.0;
                let movement_speed = 0.5;
                let mouse_sensitivity = 0.001;

                let mut camera = Camera::new(position, fov, aspect_ratio, near, far);
                camera.movement_speed = movement_speed;
                camera.mouse_sensitivity = mouse_sensitivity;

                let delta_time = 1.0;
                camera.process_keyboard(CameraMovement::Forward, delta_time);

                let expected_movement = movement_speed * delta_time;
                let expected_position = Vec3::new(0.0, 0.0, -expected_movement);

                assert!(
                    vec3_approx_eq(camera.position, expected_position, 1e-6),
                    "Expected position: {:?}, got {:?}",
                    expected_position,
                    camera.position
                );
            });

            if let Err(e) = result {
                eprintln!("Test panicked: {:?}", e);
                panic!("test_camera_movement failed");
            }
        });
        log_test_end("test_camera_movement");
    }

    #[test]
    fn test_mouse_movement() {
        log_test_start("test_mouse_movement");
        autoreleasepool(|| {
            let mut camera = Camera::new(Vec3::ZERO, 45.0, 1.0, 0.1, 100.0);
            camera.process_mouse_movement(10.0, 0.0);
            let forward = -camera.orientation * Vec3::Z;
            assert!(forward.x < 0.0); // Camera should have rotated to the left
        });
        log_test_end("test_mouse_movement");
    }
}
