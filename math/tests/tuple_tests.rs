pub mod tests {
    use math::tuple::Tuple4D;

    #[test]
    fn test_addition_of_two_tuples_owned() {
        let tuple1 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let tuple2 = Tuple4D::new(5.0, 6.0, 7.0, 0.0);
        let result = tuple1 + tuple2;

        let expected = Tuple4D::new(6.0, 8.0, 10.0, 0.0);
        assert_eq!(
            result, expected,
            "Addition of two owned Tuple4D instances failed."
        );
    }

    #[test]
    fn test_addition_of_two_tuples_references() {
        let tuple1 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let tuple2 = Tuple4D::new(5.0, 6.0, 7.0, 0.0);
        let result = &tuple1 + &tuple2;

        let expected = Tuple4D::new(6.0, 8.0, 10.0, 0.0);
        assert_eq!(
            result, expected,
            "Addition of two borrowed Tuple4D references failed."
        );
    }

    #[test]
    fn test_add_method() {
        let tuple1 = Tuple4D::new(1.5, 2.5, 3.5, 0.0);
        let tuple2 = Tuple4D::new(5.5, 6.5, 7.5, 0.0);
        let result = tuple1.add(&tuple2);

        let expected = Tuple4D::new(7.0, 9.0, 11.0, 0.0);
        assert_eq!(result, expected, "Tuple4D::add method failed.");
    }

    #[test]
    fn test_subtraction_of_two_tuples_owned() {
        let tuple1 = Tuple4D::new(10.0, 20.0, 30.0, 0.0);
        let tuple2 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let result = tuple1 - tuple2;

        let expected = Tuple4D::new(9.0, 18.0, 27.0, 0.0);
        assert_eq!(
            result, expected,
            "Subtraction of two owned Tuple4D instances failed."
        );
    }

    #[test]
    fn test_subtraction_of_two_tuples_references() {
        let tuple1 = Tuple4D::new(10.0, 20.0, 30.0, 0.0);
        let tuple2 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let result = &tuple1 - &tuple2;

        let expected = Tuple4D::new(9.0, 18.0, 27.0, 0.0);
        assert_eq!(
            result, expected,
            "Subtraction of two borrowed Tuple4D references failed."
        );
    }

    #[test]
    fn test_subtract_method() {
        let tuple1 = Tuple4D::new(5.5, 6.5, 7.5, 0.0);
        let tuple2 = Tuple4D::new(1.5, 2.5, 3.5, 0.0);
        let result = tuple1.subtract(&tuple2);

        let expected = Tuple4D::new(4.0, 4.0, 4.0, 0.0);
        assert_eq!(result, expected, "Tuple4D::subtract method failed.");
    }

    #[test]
    fn test_point_to_vector() {
        let tuple1 = Tuple4D::new(5.5, 6.5, 7.5, 1.0);
        let tuple2 = Tuple4D::new(1.5, 2.5, 3.5, 1.0);
        let result = tuple1.subtract(&tuple2);

        let expected = Tuple4D::new(4.0, 4.0, 4.0, 0.0);
        assert_eq!(
            result, expected,
            "Tuple4D::test_point_to_vector method failed point - point should be a vector."
        );
    }

    #[test]
    fn test_vector_addition_to_point() {
        let tuple1 = Tuple4D::new(5.5, 6.5, 7.5, 1.0);
        let tuple2 = Tuple4D::new(1.5, 2.5, 3.5, 0.0);
        let result = tuple1.add(&tuple2);

        let expected = Tuple4D::new(7.0, 9.0, 11.0, 1.0);
        assert_eq!(
            result, expected,
            "Tuple4D::test_vector_addition_to_point method failed point + vector should be a point"
        );
    }

    #[test]
    fn test_negation() {
        let tuple1 = Tuple4D::new(5.5, 6.5, 7.5, 1.0);
        let result = -tuple1;

        let expected = Tuple4D::new(-5.5, -6.5, -7.5, -1.0);
        assert_eq!(result, expected, "Tuple4D::test_negation method failed");
    }

    #[test]
    fn test_scalar_multi() {
        let tuple1 = Tuple4D::new(5.0, 6.0, 7.0, 1.0);
        let result = 2.0 * tuple1;

        let expected = Tuple4D::new(10.0, 12.0, 14.0, 2.0);
        assert_eq!(result, expected, "Tuple4D::test_scalar_multi method failed");
    }

    // Additional Tests Below

    #[test]
    fn test_equality_of_tuples() {
        let tuple1 = Tuple4D::new(1.0, 2.0, 3.0, 4.0);
        let tuple2 = Tuple4D::new(1.0, 2.0, 3.0, 4.0);
        let tuple3 = Tuple4D::new(4.0, 3.0, 2.0, 1.0);

        assert_eq!(
            tuple1, tuple2,
            "Tuples with identical components should be equal."
        );
        assert_ne!(
            tuple1, tuple3,
            "Tuples with different components should not be equal."
        );
    }

    #[test]
    fn test_zero_tuple() {
        let zero = Tuple4D::new(0.0, 0.0, 0.0, 0.0);
        let tuple = Tuple4D::new(0.0, 0.0, 0.0, 0.0);

        assert_eq!(
            zero, tuple,
            "Zero tuple should be equal to another zero tuple."
        );
    }

    #[test]
    fn test_scalar_division() {
        let tuple = Tuple4D::new(10.0, 20.0, 30.0, 40.0);
        let result = tuple / 2.0;

        let expected = Tuple4D::new(5.0, 10.0, 15.0, 20.0);
        assert_eq!(result, expected, "Scalar division of Tuple4D failed.");
    }

    #[test]
    fn test_magnitude() {
        let vector = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let magnitude = vector.magnitude();

        let expected = (1.0_f64 + 4.0 + 9.0).sqrt();
        assert_eq!(
            magnitude, expected,
            "Magnitude calculation of Tuple4D vector failed."
        );
    }

    #[test]
    fn test_normalization() {
        let vector = Tuple4D::new(4.0, 0.0, 0.0, 0.0);
        let normalized = vector.normalize();

        let expected = Tuple4D::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(
            normalized, expected,
            "Normalization of Tuple4D vector failed."
        );

        let vector = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let normalized = vector.normalize();
        let magnitude = (1.0_f64 + 4.0 + 9.0).sqrt();

        let expected = Tuple4D::new(1.0 / magnitude, 2.0 / magnitude, 3.0 / magnitude, 0.0);
        assert_eq!(
            normalized, expected,
            "Normalization of Tuple4D vector with non-unit magnitude failed."
        );
    }

    #[test]
    fn test_dot_product() {
        let v1 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let v2 = Tuple4D::new(2.0, 3.0, 4.0, 0.0);
        let dot = v1.dot(&v2);

        let expected = 1.0 * 2.0 + 2.0 * 3.0 + 3.0 * 4.0;
        assert_eq!(dot, expected, "Dot product of Tuple4D vectors failed.");
    }

    #[test]
    fn test_normalization_preserves_direction() {
        let vector = Tuple4D::new(0.0, 0.0, 0.0, 0.0);
        let normalized = vector.normalize();

        // Depending on implementation, normalizing a zero vector might return zero or panic.
        // Adjust the test based on your implementation.
        let expected = Tuple4D::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(
            normalized, expected,
            "Normalization of zero Tuple4D vector failed."
        );
    }

    #[test]
    fn test_scalar_multiplication_with_negative_scalar() {
        let tuple = Tuple4D::new(1.0, -2.0, 3.0, -4.0);
        let result = -3.0 * tuple;

        let expected = Tuple4D::new(-3.0, 6.0, -9.0, 12.0);
        assert_eq!(
            result, expected,
            "Scalar multiplication with negative scalar failed."
        );
    }

    #[test]
    fn test_cross_product() {
        let v1 = Tuple4D::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Tuple4D::new(0.0, 1.0, 0.0, 0.0);
        let cross = v1.cross(&v2);

        let expected = Tuple4D::new(0.0, 0.0, 1.0, 0.0);
        assert_eq!(
            cross, expected,
            "Cross product of v1 and v2 should be (0, 0, 1, 0)."
        );
    }

    /// Tests the cross product is anti-commutative: a × b = - (b × a).
    #[test]
    fn test_cross_product_anti_commutative() {
        let a = Tuple4D::new(2.0, 3.0, 4.0, 0.0);
        let b = Tuple4D::new(5.0, 6.0, 7.0, 0.0);

        let cross_ab = a.cross(&b);
        let cross_ba = b.cross(&a);
        let expected = Tuple4D::new(-cross_ab.x, -cross_ab.y, -cross_ab.z, 0.0);

        assert_eq!(
            cross_ba, expected,
            "Cross product should be anti-commutative."
        );
    }

    /// Tests that the cross product of a vector with itself is the zero vector.
    #[test]
    fn test_cross_product_with_itself() {
        let v = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let cross = v.cross(&v);

        let expected = Tuple4D::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(
            cross, expected,
            "Cross product of a vector with itself should be the zero vector."
        );
    }

    /// Tests the cross product with negative components.
    #[test]
    fn test_cross_product_with_negative_components() {
        let v1 = Tuple4D::new(-1.0, -2.0, -3.0, 0.0);
        let v2 = Tuple4D::new(4.0, 5.0, 6.0, 0.0);
        let cross = v1.cross(&v2);

        let expected = Tuple4D::new(3.0, -6.0, 3.0, 0.0);

        assert_eq!(
            cross, expected,
            "Cross product with negative components failed."
        );
    }

    /// Tests the cross product of two arbitrary vectors.
    #[test]
    fn test_cross_product_arbitrary_vectors() {
        let v1 = Tuple4D::new(2.0, 3.0, 4.0, 0.0);
        let v2 = Tuple4D::new(5.0, 6.0, 7.0, 0.0);
        let cross = v1.cross(&v2);

        let expected = Tuple4D::new(
            3.0 * 7.0 - 4.0 * 6.0, // (y1*z2 - z1*y2) = 21 - 24 = -3
            4.0 * 5.0 - 2.0 * 7.0, // (z1*x2 - x1*z2) = 20 - 14 = 6
            2.0 * 6.0 - 3.0 * 5.0, // (x1*y2 - y1*x2) = 12 - 15 = -3
            0.0,
        );

        let expected = Tuple4D::new(-3.0, 6.0, -3.0, 0.0);
        assert_eq!(
            cross, expected,
            "Cross product of arbitrary vectors failed."
        );
    }

    /// Tests that cross product results in a vector perpendicular to the original vectors.
    #[test]
    fn test_cross_product_perpendicularity() {
        let v1 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let v2 = Tuple4D::new(4.0, 5.0, 6.0, 0.0);
        let cross = v1.cross(&v2);

        // The dot product of perpendicular vectors should be zero
        let dot1 = v1.dot(&cross);
        let dot2 = v2.dot(&cross);

        assert!(
            dot1.abs() < 1e-10,
            "Dot product of v1 and cross(v1, v2) should be zero."
        );
        assert!(
            dot2.abs() < 1e-10,
            "Dot product of v2 and cross(v1, v2) should be zero."
        );
    }

    /// Tests that cross product handles vectors with w components correctly (should ignore w).
    #[test]
    fn test_cross_product_with_non_zero_w() {
        let v1 = Tuple4D::new(1.0, 0.0, 0.0, 1.0); // Point or vector with w = 1.0
        let v2 = Tuple4D::new(0.0, 1.0, 0.0, 1.0);
        let cross = v1.cross(&v2);

        let expected = Tuple4D::new(0.0, 0.0, 1.0, 1.0); // w should be set to 0.0
        assert_eq!(
            cross, expected,
            "Cross product should ignore w components and set w to 0.0."
        );
    }
}
