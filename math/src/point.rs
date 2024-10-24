use crate::tuple::Tuple4D;
use crate::vector::Vector3D;
use std::ops::{Add, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Point3D {
    pub(crate) tuple: Tuple4D,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            tuple: Tuple4D::new(x, y, z, 1.0),
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
}

// Addition for points with vectors
impl Add<&Vector3D> for &Point3D {
    type Output = Point3D;

    fn add(self, rhs_vec: &Vector3D) -> Point3D {
        Point3D {
            tuple: Tuple4D {
                x: self.x() + rhs_vec.x(),
                y: self.y() + rhs_vec.y(),
                z: self.z() + rhs_vec.z(),
                w: self.tuple.w,
            },
        }
    }
}

impl Add<&Point3D> for &Vector3D {
    type Output = Point3D;

    fn add(self, rhs: &Point3D) -> Point3D {
        rhs + self
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Point3D {
        &self + &rhs
    }
}

impl Add<Point3D> for Vector3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Point3D {
        &rhs + &self
    }
}

// Subtraction for a point and a vector
impl Sub<&Vector3D> for &Point3D {
    type Output = Point3D;

    fn sub(self, rhs_vec: &Vector3D) -> Point3D {
        Point3D {
            tuple: Tuple4D {
                x: self.x() - rhs_vec.x(),
                y: self.y() - rhs_vec.y(),
                z: self.z() - rhs_vec.z(),
                w: self.tuple.w,
            },
        }
    }
}

impl Sub<Vector3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Vector3D) -> Point3D {
        &self - &rhs
    }
}

//Subtraction two points
impl Sub<&Point3D> for &Point3D {
    type Output = Vector3D;

    fn sub(self, rhs: &Point3D) -> Vector3D {
        Vector3D::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Vector3D;

    fn sub(self, rhs: Point3D) -> Vector3D {
        &self - &rhs
    }
}
