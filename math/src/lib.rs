pub mod vector;

#[cfg(test)]
mod tests {
    use crate::vector::Vector3D;
    use super::*;

    #[test]
    fn vector_addition() {
        let result = Vector3D::new(3.0, 2.0,2.0).add(&Vector3D::new(2.0, 2.0,2.0));
        assert_eq!(result, Vector3D::new(5.0,4.0,4.0));
    }

    #[test]
    fn vector_subtraction() {
        let result = Vector3D::new(3.0, 2.0,2.0).subtract(&Vector3D::new(2.0, 2.0,2.0));
        assert_eq!(result, Vector3D::new(1.0,0.0,0.0));
    }

    #[test]
    fn vector_magnitude() {
        let result = Vector3D::new(1.0, 1.0,1.0);
        assert_eq!(result.mag(), f64::sqrt(result.x * result.x  + result.y * result.y + result.z * result.z));
    }

    // fn vector_dot_product() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
