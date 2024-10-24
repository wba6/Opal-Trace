#[cfg(test)]
mod tests {
    use math::point::Point3D;
    use math::vector::Vector3D;

    #[test]
    fn test_point_vector_addition() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        let v = Vector3D::new(4.0, 5.0, 6.0);

        let result = &p + &v;
        assert_eq!(result, Point3D::new(5.0, 7.0, 9.0));

        let result = p + v;
        assert_eq!(result, Point3D::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector_point_addition() {
        let v = Vector3D::new(4.0, 5.0, 6.0);
        let p = Point3D::new(1.0, 2.0, 3.0);

        let result = &v + &p;
        assert_eq!(result, Point3D::new(5.0, 7.0, 9.0));

        let result = v + p;
        assert_eq!(result, Point3D::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_point_vector_subtraction() {
        let v = Vector3D::new(5.0, 4.0, 6.0);
        let p = Point3D::new(1.0, 2.0, 3.0);

        let result = &p - &v;
        assert_eq!(result, Point3D::new(-4.0, -2.0, -3.0));

        let result = p - v;
        assert_eq!(result, Point3D::new(-4.0, -2.0, -3.0));
    }

    #[test]
    fn test_point_point_subtraction() {
        let p1 = Point3D::new(4.0, 5.0, 6.0);
        let p2 = Point3D::new(1.0, 2.0, 3.0);

        let result = &p2 - &p1;
        assert_eq!(result, Vector3D::new(-3.0, -3.0, -3.0));

        let result = p2 - p1;
        assert_eq!(result, Vector3D::new(-3.0, -3.0, -3.0));
    }
}
