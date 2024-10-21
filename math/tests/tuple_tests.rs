#[cfg(test)]
pub mod tests {
    use math::tuple::Tuple4D;
    #[test]
    fn test_addition_of_two_tuples_owned() {
        let tuple1 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let tuple2 = Tuple4D::new(5.0, 6.0, 7.0, 0.0);
        let result = tuple1 + tuple2;

        let expected = Tuple4D::new(6.0, 8.0, 10.0, 0.0);
        assert_eq!(result, expected, "Addition of two owned Tuple4D instances failed.");
    }

    #[test]
    fn test_addition_of_two_tuples_references() {
        let tuple1 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let tuple2 = Tuple4D::new(5.0, 6.0, 7.0, 0.0);
        let result = &tuple1 + &tuple2;

        let expected = Tuple4D::new(6.0, 8.0, 10.0, 0.0);
        assert_eq!(result, expected, "Addition of two borrowed Tuple4D references failed.");
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
        assert_eq!(result, expected, "Subtraction of two owned Tuple4D instances failed.");
    }

    #[test]
    fn test_subtraction_of_two_tuples_references() {
        let tuple1 = Tuple4D::new(10.0, 20.0, 30.0, 0.0);
        let tuple2 = Tuple4D::new(1.0, 2.0, 3.0, 0.0);
        let result = &tuple1 - &tuple2;

        let expected = Tuple4D::new(9.0, 18.0, 27.0, 0.0);
        assert_eq!(result, expected, "Subtraction of two borrowed Tuple4D references failed.");
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
        assert_eq!(result, expected, "Tuple4D::test_point_to_vector method failed point - point should be a vector.");
    }

    #[test]
    fn test_vector_addition_to_point() {
        let tuple1 = Tuple4D::new(5.5, 6.5, 7.5, 1.0);
        let tuple2 = Tuple4D::new(1.5, 2.5, 3.5, 0.0);
        let result = tuple1.subtract(&tuple2);

        let expected = Tuple4D::new(4.0, 4.0, 4.0, 1.0);
        assert_eq!(result, expected, "Tuple4D::test_vector_addition_to_point method failed point + vector should be a point");
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


}
