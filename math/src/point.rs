use crate::tuple::Tuple4D;

pub struct Point3D {
    tuple : Tuple4D
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { tuple: Tuple4D::new(x,y,z,1.0) }
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

