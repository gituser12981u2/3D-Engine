use num_traits::{Float, FromPrimitive};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Vectors
impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }
}

/// Add two vectors
impl<T: Copy + Add<Output = T>> Add for Vector3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Subtract two vectors
impl<T: Copy + Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Multiply two vectors
impl<T: Copy + Mul<Output = T>> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

/// Divide two vectors
impl<T: Copy + Div<Output = T>> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// Extended operations for floating-point types
impl<T: Copy + Float + FromPrimitive> Vector3<T> {
    pub fn zero() -> Self {
        Vector3::new(T::zero(), T::zero(), T::zero())
    }

    pub fn scale(&self, scalar: T) -> Self {
        *self * scalar
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[allow(dead_code)]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag != T::zero() {
            *self / mag
        } else {
            Vector3::zero()
        }
    }
}

// Type aliases
pub type PhysicsVector3 = Vector3<f64>;
pub type RenderVector3 = Vector3<f32>;

// Conversion implementations
impl From<PhysicsVector3> for RenderVector3 {
    fn from(v: PhysicsVector3) -> Self {
        RenderVector3::new(v.x as f32, v.y as f32, v.z as f32)
    }
}

impl From<RenderVector3> for PhysicsVector3 {
    fn from(v: RenderVector3) -> Self {
        PhysicsVector3::new(v.x as f64, v.y as f64, v.z as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_vector3_float_operations() {
        // Test with f32
        let v1_f32 = Vector3::new(1.0f32, 2.0f32, 3.0f32);
        assert!((v1_f32.magnitude() - 3.7416573f32).abs() < 1e-6);

        let normalized_f32 = v1_f32.normalize();
        assert!((normalized_f32.magnitude() - 1.0f32).abs() < 1e-6);

        // Test with f64
        let v1_f64 = Vector3::new(1.0f64, 2.0f64, 3.0f64);
        assert!((v1_f64.magnitude() - 3.7416573867739413).abs() < EPSILON);

        let normalized_f64 = v1_f64.normalize();
        assert!((normalized_f64.magnitude() - 1.0).abs() < EPSILON);
    }
}
