use super::vector3::Vector3;

#[allow(dead_code)]
#[derive(Debug)]
pub struct RigidBodySystem {
    masses: Vec<f64>,
    positions: Vec<Vector3>,
    velocities: Vec<Vector3>,
    accelerations: Vec<Vector3>,
    forces: Vec<Vector3>,
}

impl RigidBodySystem {
    #[allow(dead_code)]
    pub fn new() -> Self {
        RigidBodySystem {
            masses: Vec::new(),
            positions: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
            forces: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Self {
        RigidBodySystem {
            masses: Vec::with_capacity(capacity),
            positions: Vec::with_capacity(capacity),
            velocities: Vec::with_capacity(capacity),
            accelerations: Vec::with_capacity(capacity),
            forces: Vec::with_capacity(capacity),
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mass: f64, position: Vector3, velocity: Vector3) -> usize {
        let index = self.masses.len();
        self.masses.push(mass);
        self.positions.push(position);
        self.velocities.push(velocity);
        self.accelerations.push(Vector3::zero());
        self.forces.push(Vector3::zero());
        index
    }

    #[allow(dead_code)]
    pub fn update_verlet(&mut self, dt: f64) {
        for i in 0..self.masses.len() {
            // Calculate acceleration from continuous force
            self.accelerations[i] = self.forces[i] / self.masses[i];

            // Update position using Verlet integration
            // x = x + vt + a*0.5*t^2
            self.positions[i] +=
                self.velocities[i] * dt + self.accelerations[i].scale(0.5 * dt * dt);

            // Store old acceleration for velocity update
            let old_acceleration = self.accelerations[i];

            // Calculate new acceleration
            self.accelerations[i] = self.forces[i] / self.masses[i];

            // Update velocity using average acceleration
            // v = v + 1/2(a_0+a)* t
            self.velocities[i] += (old_acceleration + self.accelerations[i]).scale(0.5 * dt);

            // Reset forces for next iteration
            self.accelerations[i] = Vector3::zero();
        }
    }

    #[allow(dead_code)]
    pub fn update_rk4(&mut self, dt: f64, force_func: impl Fn(&Vector3, &Vector3) -> Vector3) {
        for i in 0..self.masses.len() {
            let k1v = force_func(&self.positions[i], &self.velocities[i]) / self.masses[i] * dt;
            let k1r = self.velocities[i] * dt;

            let k2v = force_func(
                &(self.positions[i] + k1r * 0.5),
                &(self.velocities[i] + k1v * 0.5),
            ) / self.masses[i]
                * dt;
            let k2r = (self.velocities[i] + k1v * 0.5) * dt;

            let k3v = force_func(
                &(self.positions[i] + k2r * 0.5),
                &(self.velocities[i] + k2v * 0.5),
            ) / self.masses[i]
                * dt;
            let k3r = (self.velocities[i] + k2v * 0.5) * dt;

            let k4v = force_func(&(self.positions[i] + k3r), &(self.velocities[i] + k3v))
                / self.masses[i]
                * dt;
            let k4r = (self.velocities[i] + k3v) * dt;

            self.velocities[i] += (k1v + k2v * 2.0 + k3v * 2.0 + k4v) * (1.0 / 6.0);
            self.velocities[i] += (k1r + k2r * 2.0 + k3r * 2.0 + k4r) * (1.0 / 6.0);
        }
    }

    #[allow(dead_code)]
    pub fn apply_force(&mut self, index: usize, force: Vector3) {
        self.forces[index] += force;
    }

    #[allow(dead_code)]
    pub fn apply_force_to_all(&mut self, force: Vector3) {
        for f in &mut self.forces {
            *f += force;
        }
    }

    // Getter methods
    #[allow(dead_code)]
    pub fn position(&self, index: usize) -> Vector3 {
        self.positions[index]
    }

    #[allow(dead_code)]
    pub fn velocity(&self, index: usize) -> Vector3 {
        self.velocities[index]
    }

    #[allow(dead_code)]
    pub fn mass(&self, index: usize) -> f64 {
        self.masses[index]
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.masses.len()
    }
}
