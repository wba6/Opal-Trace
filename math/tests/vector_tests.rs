#[cfg(test)]
pub mod tests {
    use math::vector::Vector3D;

    #[test]
    fn vector_addition_basic() {
        let v1 = Vector3D::new(3.0, 2.0, 2.0);
        let v2 = Vector3D::new(2.0, 2.0, 2.0);
        let result = v1.add(&v2);
        assert_eq!(result, Vector3D::new(5.0, 4.0, 4.0));
    }

    #[test]
    fn vector_addition_with_zero() {
        let v1 = Vector3D::new(0.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, -1.0, 3.5);
        let result = v1.add(&v2);
        assert_eq!(result, v2);
    }

    #[test]
    fn vector_addition_negative_components() {
        let v1 = Vector3D::new(-1.0, -2.0, -3.0);
        let v2 = Vector3D::new(-4.0, 5.0, -6.0);
        let result = v1.add(&v2);
        assert_eq!(result, Vector3D::new(-5.0, 3.0, -9.0));
    }

    #[test]
    fn vector_subtraction_basic() {
        let v1 = Vector3D::new(3.0, 2.0, 2.0);
        let v2 = Vector3D::new(2.0, 2.0, 2.0);
        let result = v1.subtract(&v2);
        assert_eq!(result, Vector3D::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn vector_subtraction_resulting_in_negatives() {
        let v1 = Vector3D::new(1.0, 1.0, 1.0);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);
        let result = v1.subtract(&v2);
        assert_eq!(result, Vector3D::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn vector_subtraction_with_zero() {
        let v1 = Vector3D::new(5.0, -3.0, 2.0);
        let v2 = Vector3D::new(0.0, 0.0, 0.0);
        let result = v1.subtract(&v2);
        assert_eq!(result, v1);
    }

    #[test]
    fn vector_magnitude_basic() {
        let v = Vector3D::new(1.0, 1.0, 1.0);
        let expected = f64::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
        assert_eq!(v.magnitude(), expected);
    }

    #[test]
    fn vector_magnitude_zero_vector() {
        let v = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn vector_magnitude_negative_components() {
        let v = Vector3D::new(-3.0, -4.0, 0.0);
        let expected = 5.0; // sqrt(9 + 16 + 0) = 5
        assert_eq!(v.magnitude(), expected);
    }

    #[test]
    fn vector_dot_product_basic() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        let expected = 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0; // 32.0
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_dot_product_with_zero() {
        let v1 = Vector3D::new(0.0, 0.0, 0.0);
        let v2 = Vector3D::new(7.0, -8.0, 9.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn vector_dot_product_negative_components() {
        let v1 = Vector3D::new(-1.0, -2.0, -3.0);
        let v2 = Vector3D::new(4.0, -5.0, 6.0);
        let result = v1.dot(&v2);
        let expected = (-1.0) * 4.0 + (-2.0) * (-5.0) + (-3.0) * 6.0; // -4 + 10 -18 = -12
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_dot_product_orthogonal_vectors() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn vector_dot_product_parallel_vectors() {
        let v1 = Vector3D::new(2.0, 2.0, 2.0);
        let v2 = Vector3D::new(4.0, 4.0, 4.0);
        let result = v1.dot(&v2);
        let expected = 2.0 * 4.0 + 2.0 * 4.0 + 2.0 * 4.0; // 24.0
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_basic() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        let expected = Vector3D::new(0.0, 0.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_negative_components() {
        let v1 = Vector3D::new(-1.0, -2.0, -3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);
        let result = v1.cross(&v2);
        let expected = Vector3D::new(3.0, -6.0, 3.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_parallel_vectors() {
        let v1 = Vector3D::new(2.0, 2.0, 2.0);
        let v2 = Vector3D::new(4.0, 4.0, 4.0);
        let result = v1.cross(&v2);
        let expected = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_orthogonal_vectors() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 0.0, 1.0);
        let result = v1.cross(&v2);
        let expected = Vector3D::new(0.0, -1.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_zero_vector() {
        let v1 = Vector3D::new(0.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, 2.0, 3.0);
        let result = v1.cross(&v2);
        let expected = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_with_itself() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let result = v.cross(&v);
        let expected = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_cross_product_non_orthogonal_vectors() {
        let v1 = Vector3D::new(3.0, -3.0, 1.0);
        let v2 = Vector3D::new(4.0, 9.0, 2.0);
        let result = v1.cross(&v2);
        let expected = Vector3D::new(-15.0, -2.0, 39.0);
        assert_eq!(result, expected);
    }
}
