
#[cfg(test)]
mod tests {
    use canvas::canvas::Canvas;
    use math::color::Color;


    #[test]
    fn canvas_getters() {
        let canvas1 = Canvas::new(65,89);

        let result = Canvas::new(canvas1.width(),canvas1.height());

        assert_eq!(result, Canvas::new(65,89));
    }

    #[test]
    fn canvas_set_pixel() {
        let mut canvas1 = Canvas::new(65,89);
        canvas1.write_pixel(34, 89, &Color::new(0.5, 0.1, 0.9));

        let expected = Color::new(0.5, 0.1, 0.9);
        assert_eq!(expected, canvas1.get_pixel(34,89));
        assert_eq!(Color::new(0.0, 0.0, 0.0), canvas1.get_pixel(33,89));
    }

    //@TODO need tests to check functionality of ppm functions
}
