use std::vec;
use std::fs::File;
use std::io::Write;
use crate::color::Color;

pub struct Canvas {
    width: i64,
    height: i64,
    color_buffer: Vec<Color>
}


impl Canvas {
    pub fn new(width: i64, height: i64) -> Canvas {
        let color_buf = vec![Color::new(0.0, 0.0, 0.0); (width * height )as usize];
        Canvas {
            width: width,
            height: height,
            color_buffer: color_buf
        }
    }

    pub fn set_pixel_at(&mut self, width: i64, height: i64, color: Color) -> Color {
        if(width < 0 || width >= self.width || height < 0 || height >= self.height) {
            panic!("Out of bounds!");
        }
        self.color_buffer[ (height * self.width + width) as usize ] = color;
        color
    }

    pub fn get_pixel_at(&self, width: i64, height: i64) -> &Color {
        if(width < 0 || width >= self.width || height < 0 || height >= self.height) {
            panic!("Out of bounds!");
        }
        &self.color_buffer[ (height * self.width + width) as usize ]
    }

    pub fn flush_ppm(&self, path: String) -> bool {
        let mut ppm_file = File::create(path);

        let mut opened_file = match ppm_file {
            Err(e) => panic!("File failed to open, {:?}", e),
            Ok(file ) => file 
        };
            
        let ppm_header: String  = format!("P3\n{} {}\n255\n", self.width, self.height);

        opened_file.write(ppm_header.as_bytes()).expect("Failed to write header!");

        for color in self.color_buffer.iter() {
            let color_str: String = format!("{} {} {}\n", color.r(), color.g(), color.b());

            opened_file.write(color_str.as_bytes()).expect("Failed to write color!");
        }

        true 
    }

}


#[cfg(test)]
mod tests {

    use crate::Canvas;
    use crate::Color;

    #[test]
    fn rg_checkerboard() {
        let WIDTH: i64 = 7 as i64;
        let HEIGHT: i64 = 7 as i64;

        let mut canv = Canvas::new(WIDTH, HEIGHT);

        for x in 0..(HEIGHT as i64) {
            for y in 0..(WIDTH as i64) {
                let mut color: Color;

                if (x + y) % 2 == 0 {
                    color = Color::new(255.0, 0.0, 0.0)        
                } else {
                    color = Color::new(0.0, 255.0, 0.0)
                }

                canv.set_pixel_at(x, y, color);
            } 
        }

        canv.flush_ppm(String::from("checkerboard.ppm"));
    }

    #[test]
    fn rb_vertical_stripes() {
        //TODO: redo this test
        let WIDTH: i64 = 10 as i64;
        let HEIGHT: i64 = 15 as i64;

        let mut canv = Canvas::new(WIDTH, HEIGHT);

        for x in 0..(WIDTH as i64) {
            for y in 0..(HEIGHT as i64) {
                let mut color: Color;

                if x % 2 == 0 {
                    color = Color::new(255.0, 0.0, 0.0)        
                } else {
                    color = Color::new(0.0, 255.0, 0.0)
                }

                canv.set_pixel_at(x, y, color);
            } 
        }
        canv.flush_ppm(String::from("vertical.ppm"));
    }

    #[test]
    fn gb_horizontal_stripes() {
        //TODO: redo this test
        let WIDTH: i64 = 15 as i64;
        let HEIGHT: i64 = 10 as i64;

        let mut canv = Canvas::new(WIDTH, HEIGHT);

        for x in 0..(WIDTH as i64) {
            for y in 0..(HEIGHT as i64) {
                let mut color: Color;

                if y % 2 == 0 {
                    color = Color::new(255.0, 0.0, 0.0)        
                } else {
                    color = Color::new(0.0, 255.0, 0.0)
                }

                canv.set_pixel_at(x, y, color);
            } 
        }
        canv.flush_ppm(String::from("horizontal.ppm"));
    }

}