use crate::common::vector3::PhysicsVector3 as Vector3;

#[allow(dead_code)]
#[derive(Debug)]
pub struct RigidBody {
    pub mass: f64,
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
}

impl RigidBody {
    #[allow(dead_code)]
    pub fn new(mass: f64, position: Vector3, velocity: Vector3) -> Self {
        RigidBody {
            mass,
            position,
            velocity,
            acceleration: Vector3::zero(),
        }
    }

    #[allow(dead_code)]
    pub fn update_verlet(&mut self, dt: f64, force_func: impl Fn(&Vector3) -> Vector3) {
        // Calculate acceleration from continuous force
        self.acceleration = force_func(&self.position) / self.mass;

        // Update position using Verlet integration
        // x = x + vt + a*0.5*t^2
        self.position = self.position + self.velocity * dt + self.acceleration.scale(0.5 * dt * dt);

        // Calculate new acceleration
        let new_acceleration = force_func(&self.position) / self.mass;

        // Update velocity using average acceleration
        // v = v + 1/2(a_0+a)* t
        self.velocity = self.velocity + (self.acceleration + new_acceleration).scale(0.5 * dt);
        // Update acceleration
        self.acceleration = new_acceleration;
    }

    #[allow(dead_code)]
    pub fn update_rk4(&mut self, dt: f64, force_func: impl Fn(&Vector3, &Vector3) -> Vector3) {
        let k1v = force_func(&self.position, &self.velocity) / self.mass * dt;
        let k1r = self.velocity * dt;

        let k2v =
            force_func(&(self.position + k1r * 0.5), &(self.velocity + k1v * 0.5)) / self.mass * dt;
        let k2r = (self.velocity + k1v * 0.5) * dt;

        let k3v =
            force_func(&(self.position + k2r * 0.5), &(self.velocity + k2v * 0.5)) / self.mass * dt;
        let k3r = (self.velocity + k2v * 0.5) * dt;

        let k4v = force_func(&(self.position + k3r), &(self.velocity + k3v)) / self.mass * dt;
        let k4r = (self.velocity + k3v) * dt;

        self.velocity = self.velocity + (k1v + k2v * 2.0 + k3v * 2.0 + k4v) * (1.0 / 6.0);
        self.velocity = self.velocity + (k1r + k2r * 2.0 + k3r * 2.0 + k4r) * (1.0 / 6.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::vector3::PhysicsVector3 as Vector3, physics::rigid_body::RigidBody};

    #[test]
    fn test_rigid_body_basic() {
        let mut body = RigidBody::new(
            1.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );

        // Tets initial state
        assert_eq!(body.mass, 1.0);
        assert_eq!(body.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(body.velocity, Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(body.acceleration, Vector3::zero());

        // Test update with no forces
        body.update_verlet(1.0, |_| Vector3::zero());
        assert_eq!(body.position, Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(body.velocity, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rigid_body_with_constant_force() {
        let mut body = RigidBody::new(1.0, Vector3::zero(), Vector3::zero());
        let force = Vector3::new(0.0, -9.8, 0.0); // Gravity-like force

        body.update_verlet(1.0, |_| force);

        // After 1 second, should have fallen approximately 4.9 meters
        assert!((body.position.y + 4.9).abs() < 0.1);
        assert!((body.velocity.y + 9.8).abs() < 0.1);
    }

    #[test]
    fn test_rigid_body_with_spring_force() {
        let initial_position = 1.0;
        let mut body = RigidBody::new(1.0, Vector3::new(1.0, 0.0, 0.0), Vector3::zero());
        let spring_constant = 10.0; // k

        // Spring force: F = -kx
        let spring_force =
            |pos: &Vector3| -> Vector3 { Vector3::new(-spring_constant * pos.x, 0.0, 0.0) };

        let mut max_x: f64 = body.position.x;
        let mut min_x: f64 = body.position.x;

        // Simulate for a short time
        for _ in 0..1000 {
            body.update_verlet(0.01, spring_force);
            max_x = max_x.max(body.position.x);
            min_x = min_x.min(body.position.x);
        }

        println!("Max x: {}, Min x: {}", max_x, min_x);
        println!(
            "Final position: {:?}, Final velocity: {:?}",
            body.position, body.velocity
        );

        // Body should have oscillated
        assert!(
            max_x <= initial_position,
            "Max x should be positive, got {max_x}"
        );
        assert!(min_x < 0.0, "Min x should be negative, got{min_x}");
        assert!(
            (max_x - min_x).abs() > 1.5,
            "Max x should be less than initial position, got {max_x}"
        );

        // Check that the body is still moving (not at rest)
        assert!(
            body.velocity.x.abs() > 0.01,
            "Body should still be moving, velocity x is {}",
            body.velocity.x
        );

        // Check that the final position is reasonable
        assert!(
            body.position.x.abs() < initial_position,
            "Final position should be less than initial, got {}",
            body.position.x
        );
    }

    #[test]
    fn test_edge_cases() {
        // Test with very large mass
        let mut large_mass_body = RigidBody::new(1e20, Vector3::zero(), Vector3::zero());
        large_mass_body.update_verlet(1.0, |_| Vector3::new(1.0, 0.0, 0.0));
        assert!(large_mass_body.velocity.magnitude() < 1e-10);

        // Test with very small mass
        let mut small_mass_body = RigidBody::new(1e-20, Vector3::zero(), Vector3::zero());
        small_mass_body.update_verlet(1.0, |_| Vector3::new(1.0, 0.0, 0.0));
        assert!(small_mass_body.velocity.magnitude() > 1e10);

        // Test with very small time step
        let mut body = RigidBody::new(1.0, Vector3::zero(), Vector3::zero());
        body.update_verlet(1e-10, |_| Vector3::new(1.0, 0.0, 0.0));
        assert!(body.position.magnitude() < 1e-10);

        // Test with very large time step
        body.update_verlet(1e10, |_| Vector3::new(1.0, 0.0, 0.0));
        assert!(body.position.magnitude() > 1e10);
    }
}
