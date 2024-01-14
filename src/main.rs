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


use crate::camera::{OrthCamera, ProjCamera, Camera};
use crate::canvas::Canvas;
use crate::sphere::Sphere;
use crate::material::Material;
use crate::vec::Vec;
use crate::color::Color;
use std::rc::Rc;
use crate::shape::ShadeContext;

const WIDTH: i64 = 10 as i64;
const HEIGHT: i64 = 5 as i64;
const NUM_SAMPLES: i8 = 10; // super-sampling

pub fn main() {

    let mut canv = Canvas::new(WIDTH, HEIGHT);
    // let cam = ProjCamera::new(2.0);

    let cam = OrthCamera::new();

    let material = Rc::new(Material::Matte(Color::new(1.0, 0.0, 0.0)));
    let sphere: Sphere = Sphere::new(1.0, Vec::new([0.0, 0.0, 1.0]), material);

    for y in 0..(HEIGHT as i64) {
        for x in 0..(WIDTH as i64) {
            //let r = cam.cast_ray(x as f64, y as f64);
            let r = cam.cast_ray(x as f64, y as f64);


            println!("{:?}", r);
        } 
    }

    canv.flush_ppm(String::from("image.ppm"));
    
}

