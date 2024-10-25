use std::fs;
use std::fs::File;
use std::io::Write;
use math::color::Color;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Canvas {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<Vec<Color>>
}

impl Canvas {

    pub fn new(width: u32, height: u32) -> Self {
        // Create a single row initialized with black pixels
        let row = vec![Color::new(0.0, 0.0, 0.0); width as usize + 1];

        // Create all rows by cloning the single row
        let pixels = vec![row; height as usize + 1];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: &Color) {
        if x > self.width || self.height < y { panic!("Out of bounds error, x:{} and y:{} must be within the canvas({},{})", x,y, self.width, self.height);  }
        self.pixels[y as usize][x as usize] = color.clone();
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x > self.width || self.height < y { panic!("Out of bounds error, x:{} and y:{} must be within the canvas({},{})", x,y, self.width, self.height);  }
        self.pixels[y as usize][x as usize].clone()
    }

    fn create_ppm_file(&self, file_name: String) -> File {
        let mut file = fs::File::create(file_name).unwrap();
        return file
    }
    pub fn write_to_ppm_file(&self, file_name: String) {
        let mut file = self.create_ppm_file(file_name);
        let ppm = self.to_ppm();
        file.write_all(ppm.as_ref()).expect("Failed to write to file");
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::from(format!("P3\n{} {}\n255\n", self.width, self.height));

        for row in self.pixels.iter().rev() {
            for column in row.iter() {
                ppm.push_str(&format!("{} {} {}\n", column.r(), column.g(), column.b()));
            }
        }
        return ppm;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

}
