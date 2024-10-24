use math::color::Color;

pub struct Canvas {}

impl Canvas {
    pub fn write_pixel(&self, x: i32, y: i32, color: Color) {panic!("Function not implemented")}
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        Color::new(0.0, 0.0, 0.0);
        panic!("Function not implemented")
    }
}
