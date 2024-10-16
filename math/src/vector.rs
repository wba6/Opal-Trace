use std::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Vector3D{
    pub(crate) x : f64,
    pub(crate) y : f64,
    pub(crate) z : f64
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

impl Vector3D {

    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // Vector addition
    pub fn add(&self, vec2 : &Vector3D) -> Vector3D{
        return self + &vec2;
    }

    // Vector subtraction
    pub fn subtract(&self, vec2 : &Vector3D) -> Vector3D{
        return self - vec2;
    }

    // Dot Product
    pub fn dot(&self, vec2 : &Vector3D) -> f64{
        return (self.x * vec2.x) + (self.y * vec2.y) + (self.z * vec2.z);
    }

    // magnitude
    pub fn mag(&self) -> f64 {
        return f64::sqrt( (self.x * self.x) + (self.y * self.y) + (self.z * self.z));
    }

}