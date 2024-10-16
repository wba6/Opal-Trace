// Declare the math module (which refers to the math directory)
mod Math {
    // Declare the vector3D submodule
    pub mod vector;
}

// Import the Vector3D structure from the math::vector3D module
use Math::vector::Vector3D;

fn main() {
    println!("Hello, world!");
    let v1 : Vector3D = Vector3D::new(1.0,2.0,5.0);
    panic!("vector made!!!")
}
