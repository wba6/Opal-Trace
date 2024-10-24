use crate::tuple::Tuple4D;
use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Vector3D {
    pub(crate) tuple: Tuple4D,
}

impl Vector3D {
    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            tuple: Tuple4D::new(x, y, z, 0.0),
        }
    }

    pub fn x(&self) -> f64 {
        self.tuple.x
    }

    pub fn y(&self) -> f64 {
        self.tuple.y
    }

    pub fn z(&self) -> f64 {
        self.tuple.z
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
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    // Magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    // Cross Product
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            tuple: Tuple4D {
                x: self.y() * other.z() - self.z() * other.y(),
                y: self.z() * other.x() - self.x() * other.z(),
                z: self.x() * other.y() - self.y() * other.x(),
                w: self.tuple.w,
            },
        }
    }

    pub fn multi_component_wise(&self, rhs: &Vector3D) -> Vector3D {
        Vector3D {
            tuple: Tuple4D {
                x: self.x() * rhs.x(),
                y: self.y() * rhs.y(),
                z: self.z() * rhs.z(),
                w: self.tuple.w,
            },
        }
    }
}

// Implement Add for references to Vector3D
impl Add for &Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: &Vector3D) -> Vector3D {
        Vector3D {
            tuple: Tuple4D {
                x: self.x() + rhs.x(),
                y: self.y() + rhs.y(),
                z: self.z() + rhs.z(),
                w: self.tuple.w,
            },
        }
    }
}

// Implement Sub for references to Vector3D
impl Sub for &Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: &Vector3D) -> Vector3D {
        Vector3D {
            tuple: Tuple4D {
                x: self.x() - rhs.x(),
                y: self.y() - rhs.y(),
                z: self.z() - rhs.z(),
                w: self.tuple.w,
            },
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
            tuple: Tuple4D {
                x: self.x() * scalar,
                y: self.y() * scalar,
                z: self.z() * scalar,
                w: self.tuple.w,
            },
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Vector3D {
        &self * scalar
    }
}
