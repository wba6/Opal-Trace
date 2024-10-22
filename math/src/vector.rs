use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: &Self) -> Self {
        self + other
    }

    pub fn sub(&self, other: &Self) -> Self {
        self - other
    }

    pub fn scale(&self, scalar: f64) -> Self {
        self * scalar
    }

    // Dot Product
    pub fn dot(&self, other: &Vector3D) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // Magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Cross Product
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn multi_component_wise(&self, rhs: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// Implement Add for references to Vector3D
impl Add for &Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement Sub for references to Vector3D
impl Sub for &Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Implement Add and Sub for owned Vector3D, delegating to the reference implementations
impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Vector3D {
        &self + &rhs
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Vector3D {
        &self - &rhs
    }
}

// Implement scalar multiplication for Vector3D
impl Mul<f64> for &Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Vector3D {
        &self * scalar
    }
}
