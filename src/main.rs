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
mod triangle;
mod plane;
mod bvh;

use std::i8::MAX;

use shape::Shape;
use world::World;

use crate::camera::{OrthCamera, ProjCamera, Camera, AliasMode};
use crate::canvas::Canvas;
use crate::color::Color;
use crate::world::{trace};
use crate::scenes::scene1;
use crate::instance::Instance;

const WIDTH: i64 = 1000 as i64;
const HEIGHT: i64 = 500 as i64;
const NUM_SAMPLES: i8 = 20; // super-sampling
const MAX_DEPTH: i8 = 10;

pub fn main() {
    let mut canv = Canvas::new(WIDTH, HEIGHT);

    let cam = ProjCamera::new(5.0);

    let world = scene1();

    // println!("BEGIN=============");
    for y in 0..(HEIGHT as i64) {
      eprint!("\rNumber of scanlines remaining: {:?}", HEIGHT - 1 - y);
        for x in 0..(WIDTH as i64) {
           //  println!("B===========================");

           if y == 0 && x == 8 {
            print!("Breakpoint here1");
           }

            // let mut color = Color::new(0.0, 0.0, 0.0);
            // for i in 0..NUM_SAMPLES {
            //     let r = cam.cast_ray(x as f64, y as f64, AliasMode::AntiAliasOn);
            
            //     color += trace(&r, &world, MAX_DEPTH, 0.0);
            // }
            // color /= (NUM_SAMPLES as f64);


            let r = cam.cast_ray(x as f64, y as f64, AliasMode::AntiAliasOff);
          
            // println!("Ray: {:?}",r);
            
            let color = trace(&r, &world, MAX_DEPTH, 0.0);
            
            // println!("Color: {:?}",color);
            // println!("E===========================");
            canv.set_pixel_at(x, y, color);
        } 
    }
    print!("\n");
    canv.flush_ppm(String::from("rust_image.ppm"));
    
}

