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

use crate::canvas::Canvas;

const WIDTH: f64 = 1000.0 as f64;
const HEIGHT: f64 = 1000.0 as f64;
const NUM_SAMPLES: i8 = 10; // super-sampling

pub fn main() {

    let mut canv: Canvas = Canvas::new(WIDTH, HEIGHT);

    for x in 0..(HEIGHT as i64) {
        for y in 0..(WIDTH as i64) {
            println!("x: {}, y: {}", x, y)
        } 
    }
    
}

