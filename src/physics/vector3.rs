use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Vectors
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }

    #[allow(dead_code)]
    pub fn one() -> Self {
        Vector3::new(1.0, 1.0, 1.0)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        *self * scalar
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[allow(dead_code)]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag != 0.0 {
            *self / mag
        } else {
            Vector3::zero()
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Subtract two vectors
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Multiply two vectors
impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

/// Divide two vectors
impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::vector3::Vector3;
    use std::f64::EPSILON;

    #[test]
    fn test_vector3_operations() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);

        // Test addition
        let sum = v1 + v2;
        assert_eq!(sum, Vector3::new(5.0, 7.0, 9.0));

        // Test subtraction
        let diff = v2 - v1;
        assert_eq!(diff, Vector3::new(3.0, 3.0, 3.0));

        // Test multiplication by scalar
        let scaled = v1 * 2.0;
        assert_eq!(scaled, Vector3::new(2.0, 4.0, 6.0));

        // Test scalar multiplication
        let scaled2 = v1.scale(2.0);
        assert_eq!(scaled2, Vector3::new(2.0, 4.0, 6.0));

        // Test division by scalar
        let divided = v1 / 2.0;
        assert_eq!(divided, Vector3::new(0.5, 1.0, 1.5));

        // Test magnitude
        let mag = v1.magnitude();
        assert!((mag - 3.7416573867739413).abs() < EPSILON);

        // Test normalize
        let normalized = v1.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < EPSILON);
        assert!((normalized.x - 0.2672612419124244).abs() < EPSILON);
        assert!((normalized.y - 0.5345224838248488).abs() < EPSILON);
        assert!((normalized.z - 0.8017837257372732).abs() < EPSILON);
    }
}
