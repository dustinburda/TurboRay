#![feature(generic_const_exprs)]

mod vec;
mod sphere;
mod ray;
mod shape;
mod camera;
mod canvas;
mod color;
mod material;
mod shading;
mod bbox;
mod world;

use color::Color;

use crate::canvas::Canvas;

const WIDTH: i64 = 7 as i64;
const HEIGHT: i64 = 7 as i64;
const NUM_SAMPLES: i8 = 10; // super-sampling

pub fn main() {

    let mut canv = Canvas::new(WIDTH, HEIGHT);

    for x in 0..(WIDTH as i64) {
        for y in 0..(HEIGHT as i64) {
            let mut color: Color;

            if (x + y) % 2 == 0 {
                color = Color::new(255.0, 0.0, 0.0)        
            } else {
                color = Color::new(0.0, 255.0, 0.0)
            }

            canv.set_pixel_at(x, y, color);
        } 
    }

    canv.flush_ppm(String::from("image.ppm"));
    
}

