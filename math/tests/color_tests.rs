#[cfg(test)]
pub mod tests {
    use math::color::Color;

    #[test]
    fn color_vector_components() {
        let color = Color::new(1.0, 2.0, 3.0);
        let result = Color::new(1.0, 2.0, 3.0);
        assert_eq!(color, result);
    }

    #[test]
    fn color_addition_components() {
        let color = Color::new(1.0, 2.0, 3.0);
        let color2 = Color::new(1.0, 2.0, 3.0);

        let result = color.add(&color2);
        let expected = Color::new(2.0, 4.0, 6.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn color_subtraction_components() {
        let color = Color::new(1.0, 2.0, 3.0);
        let color2 = Color::new(3.0, 5.0, 6.0);

        let result = color.sub(&color2);
        let expected = Color::new(-2.0, -3.0, -3.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn color_multi_components() {
        let color = Color::new(1.0, 2.0, 3.0);
        let color2 = Color::new(3.0, 5.0, 6.0);

        let result = color.multi(&color2);
        let expected = Color::new(3.0, 10.0, 18.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn color_multi_by_scalar() {
        let color = Color::new(6.0, 2.0, 3.0);

        let result = 2.0 * color;
        let expected = Color::new(12.0, 4.0, 6.0);
        assert_eq!(result, expected);
    }
}
