use math::color::Color;

pub struct Canvas {
    height: i32,
    width: i32
}

impl Canvas {

    pub fn new(height: i32, width: i32) -> Self {
        Self { height, width }
    }
    
    pub fn write_pixel(&self, x: i32, y: i32, color: Color) {
        panic!("Function not implemented");
    }
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        panic!("Function not implemented");
        Color::new(0.0, 0.0, 0.0);
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

}
