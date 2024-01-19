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
mod matrix;
mod transformations;
mod scenes;
mod instance;
mod light;

use shape::Shape;
use world::World;

use crate::camera::{OrthCamera, ProjCamera, Camera, AliasMode};
use crate::canvas::Canvas;
use crate::color::Color;
use crate::world::{trace};
use crate::scenes::scene1;
use crate::shading::diffuse_shading;

const WIDTH: i64 = 1000 as i64;
const HEIGHT: i64 = 500 as i64;
const NUM_SAMPLES: i8 = 20; // super-sampling

pub fn main() {

    let mut canv = Canvas::new(WIDTH, HEIGHT);

    let cam = ProjCamera::new(2.0);

    let world = scene1();

    for y in 0..(HEIGHT as i64) {
        print!("\rNumber of scanlines remaining: {:?}", HEIGHT - 1 - y);
        for x in 0..(WIDTH as i64) {
 
            // let mut color = Color::new(0.0, 0.0, 0.0);
            // for i in 0..NUM_SAMPLES {
            //     let r = cam.cast_ray(x as f64, y as f64, AliasMode::AntiAliasOn);
            
            //     color += trace(&r, &world);
            // }
            // color /= (NUM_SAMPLES as f64);


            let r = cam.cast_ray(x as f64, y as f64, AliasMode::AntiAliasOff);
            
            let color = trace(&r, &world);
            

            canv.set_pixel_at(x, y, color);
        } 
    }
    print!("\n");
    canv.flush_ppm(String::from("image.ppm"));
    
}

