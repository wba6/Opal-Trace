use crate::vector::Vector3D;
use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Color {
    vector: Vector3D,
}

impl Color {
    /// Constructor for Color
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            vector: Vector3D::new(r, g, b),
        }
    }

    /// Accessor for the red component
    pub fn r(&self) -> f64 {
        self.vector.x
    }

    /// Accessor for the green component
    pub fn g(&self) -> f64 {
        self.vector.y
    }

    /// Accessor for the blue component
    pub fn b(&self) -> f64 {
        self.vector.z
    }

    pub fn add(&self, other: &Self) -> Color {
        self + other
    }

    pub fn sub(&self, other: &Self) -> Color {
        self - other
    }

    pub fn multi(&self, other: &Self) -> Color {
        self * other
    }

    pub fn scale(&self, scalar: f64) -> Color {
        self * scalar
    }
}

// Implement Add for references to Color
impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        Color {
            vector: &self.vector + &rhs.vector,
        }
    }
}

// Implement Sub for references to Color
impl Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: &Color) -> Color {
        Color {
            vector: &self.vector - &rhs.vector,
        }
    }
}

// Implement Add and Sub for owned Color, delegating to references
impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        &self + &rhs
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        &self - &rhs
    }
}

// Implement scalar multiplication for Color
impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {
            vector: &self.vector * scalar,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {vector : &self.vector * scalar}
    }
}

// Implement scalar multiplication for Color
impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color {
            vector: &rhs.vector * self,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            vector: rhs.vector * self,
        }
    }
}

// Implement multiplication for Color
impl Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color {
            vector: self.vector.multi_component_wise(&rhs.vector)
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            vector: self.vector.multi_component_wise(&rhs.vector)
        }
    }
}
