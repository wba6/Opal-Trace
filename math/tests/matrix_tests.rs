pub mod tests {
    use math::matrix::Matrix4x4;

    #[test]
    fn test_addition_of_two_matrices_owned() {
        let m1 = Matrix4x4::identity();
        let m2 = Matrix4x4::identity();
        let result = m1 + m2;

        let expected = Matrix4x4::identity() * 2.0;
        assert_eq!(result, expected, "Matrix Addition failed.");
    }
    #[test]
    fn test_identity() {
        let result = Matrix4x4::identity();

        let expected = Matrix4x4::from_array([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(result, expected, "Matrix identity failed.");
    }

    #[test]
    fn test_identity_transpose() {
        let m1 = Matrix4x4::identity();
        let result = m1.transpose();

        let expected = Matrix4x4::from_array([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(result, expected, "Matrix identity transpose failed.");
    }

    #[test]
    fn test_transpose() {
        let m1 = Matrix4x4::from_array([
            [1.0, 3.0, 0.0, 6.0],
            [2.0, 1.0, 4.0, 0.0],
            [0.0, 1.0, 1.0, 5.0],
            [6.0, 0.0, 0.0, 1.0],
        ]);

        let result = m1.transpose();

        let expected = Matrix4x4::from_array([
            [1.0, 2.0, 0.0, 6.0],
            [3.0, 1.0, 1.0, 0.0],
            [0.0, 4.0, 1.0, 0.0],
            [6.0, 0.0, 5.0, 1.0],
        ]);

        assert_eq!(result, expected, "Matrix transpose failed.");
    }

    #[test]
    fn test_identity_determinant() {
        let result = Matrix4x4::identity().determinant();

        let expected = 1.0;
        assert_eq!(result, expected, "Matrix identity determinant failed.");
    }

    #[test]
    fn test_determinant() {
        let result = Matrix4x4::from_array([
            [2.0, 1.0, 5.0, 0.0],
            [3.0, 9.0, 8.0, 3.0],
            [2.0, 6.0, 8.0, 1.0],
            [5.0, 8.0, 2.0, 8.0],
        ])
        .determinant();

        let expected = 69.0;
        assert_eq!(result, expected, "Matrix determinant failed.");
    }

    #[test]
    fn test_inverse() {
        let result = Matrix4x4::from_array([
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 4.0, 3.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
        .inverse()
        .expect("Failed to take inverse");

        let expected = Matrix4x4::from_array([
            [1.0, -2.0, 5.0, -8.0],
            [0.0, 1.0, -4.0, 5.0],
            [0.0, 0.0, 1.0, -2.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(result, expected, "Matrix determinant failed.")
    }

    #[test]
    fn test_scalar_mul() {
        let result = Matrix4x4::identity() * 2.0;

        let expected = Matrix4x4::from_array([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 2.0],
        ]);
        assert_eq!(result, expected, "Matrix scalar multiplication failed.")
    }

    #[test]
    fn test_sub() {
        let result = Matrix4x4::identity() - Matrix4x4::identity();

        let expected = Matrix4x4::new();

        assert_eq!(result, expected, "Matrix subtraction failed.")
    }

    #[test]
    fn test_mul() {
        let a = Matrix4x4::from_array([
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 4.0, 3.0],
            [0.0, 0.0, 1.0, 2.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Matrix4x4::from_array([
            [2.0, 0.0, 1.0, 0.0],
            [3.0, 1.0, 0.0, 2.0],
            [4.0, 2.0, 1.0, 3.0],
            [5.0, 3.0, 2.0, 1.0],
        ]);

        // Expected product computed manually
        let expected = Matrix4x4::from_array([
            [40.0, 20.0, 12.0, 17.0],
            [34.0, 18.0, 10.0, 17.0],
            [14.0, 8.0, 5.0, 5.0],
            [5.0, 3.0, 2.0, 1.0],
        ]);

        let result = a * b;

        assert_eq!(result, expected, "Matrix multi failed.")
    }
}
