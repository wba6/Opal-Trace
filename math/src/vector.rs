
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Vector3D{
    x : f64,
    y : f64,
    z : f64
}
impl Vector3D {

    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // Vector addition
    pub fn add(&self, vec2 : &Vector3D) -> Vector3D{
        return Vector3D { x: self.x + vec2.x,
            y: self.y + vec2.y,
            z: self.z + vec2.z}
    }

    // Vector subtraction
    pub fn subtract(&self, vec2 : &Vector3D) -> Vector3D{
        return Vector3D { x: self.x - vec2.x,
            y: self.y - vec2.y,
            z: self.z - vec2.z}
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