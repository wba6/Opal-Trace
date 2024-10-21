use std::ops::{Add, Sub, Neg, Mul};

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Tuple4D{
    pub(crate) x : f64,
    pub(crate) y : f64,
    pub(crate) z : f64,
    w : f64, //this value is used to determine if this is a vector or a point 1.0 means point 0.0 vector
}

// Implement Add for references to Tuple4D
impl Add for &Tuple4D {
    type Output = Tuple4D;

    fn add(self, rhs: &Tuple4D) -> Tuple4D {
        return Tuple4D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

// Implement Sub for references to Tuple4D
impl Sub for &Tuple4D {
    type Output = Tuple4D;

    fn sub(self, rhs: &Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl Mul<&f64> for &Tuple4D {
    type Output = Tuple4D;

    fn mul(self, rhs: &f64) -> Tuple4D {
        Tuple4D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl Mul<&Tuple4D> for &f64 {
    type Output = Tuple4D;

    fn mul(self, rhs: &Tuple4D) -> Tuple4D {
        Tuple4D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w
        }
    }
}

impl Mul<f64> for Tuple4D {
    type Output = Tuple4D;

    fn mul(self, rhs: f64) -> Tuple4D {
        &self * &rhs
    }
}
impl Mul<Tuple4D> for f64 {
    type Output = Tuple4D;

    fn mul(self, rhs: Tuple4D) -> Tuple4D {
        &self * &rhs
    }
}

// Implement Sub for references to Tuple4D
impl Neg for &Tuple4D {
    type Output = Tuple4D;

    fn neg(self) -> Self::Output {
        Tuple4D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl Add for Tuple4D {
    type Output = Tuple4D;

    fn add(self, rhs: Tuple4D) -> Tuple4D {
        &self + &rhs
    }
}

impl Sub for Tuple4D {
    type Output = Tuple4D;

    fn sub(self, rhs: Tuple4D) -> Tuple4D {
        &self - &rhs
    }
}

impl Neg for Tuple4D {
    type Output = Tuple4D;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Tuple4D {

    // Constructor
    pub fn new(x: f64, y: f64, z: f64, w :f64) -> Self {
        Self { x, y, z, w}
    }

    // Vector addition
    pub fn add(&self, vec2 : &Tuple4D) -> Tuple4D{
        return self + &vec2;
    }

    // Vector subtraction
    pub fn subtract(&self, vec2 : &Tuple4D) -> Tuple4D{
        return self - vec2;
    }

    //scalar multi
    pub fn scale(&self, scalar : f64) -> Tuple4D {return self * &scalar;}

    // Dot Product
    pub fn dot(&self, vec2 : &Tuple4D) -> f64{
        return (self.x * vec2.x) + (self.y * vec2.y) + (self.z * vec2.z) + (self.w * vec2.w);
    }

    // magnitude
    pub fn magnitude(&self) -> f64 {
        return f64::sqrt( (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w));
    }

    pub fn is_point(&self) -> bool {
        if self.w == 0.0 {return false;};
        return true;
    }
}