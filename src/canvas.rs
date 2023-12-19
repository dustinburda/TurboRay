use std::vec;
use std::fs::File;
use std::io::Write;
use crate::color::Color;

pub struct Canvas {
    width: f64,
    height: f64,
    color_buffer: Vec<Color>
}


impl Canvas {
    pub fn new(width: f64, height: f64) -> Canvas {
        let color_buf = Vec::new();
        Canvas {
            width: width,
            height: height,
            color_buffer: color_buf
        }
    }

    pub fn set_pixel_at(&mut self, width: f64, height: f64, color: Color) -> Color {
        if(width < 0.0 || width >= self.width || height < 0.0 || height >= self.height) {
            panic!("Out of bounds!");
        }
        self.color_buffer[ (height * self.width + width) as usize ] = color;
        color
    }

    pub fn get_pixel_at(&self, width: f64, height: f64) -> &Color {
        if(width < 0.0 || width >= self.width || height < 0.0 || height >= self.height) {
            panic!("Out of bounds!");
        }
        &self.color_buffer[ (height * self.width + width) as usize ]
    }

    pub fn flush() -> bool {
        false
    }

}