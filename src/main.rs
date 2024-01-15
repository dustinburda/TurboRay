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


use shape::Shape;

use crate::camera::{OrthCamera, ProjCamera, Camera};
use crate::canvas::Canvas;
use crate::sphere::Sphere;
use crate::material::Material;
use crate::vec::Vec;
use crate::color::Color;
use std::rc::Rc;
use crate::material::ShadeContext;

const WIDTH: i64 = 100 as i64;
const HEIGHT: i64 = 100 as i64;
const NUM_SAMPLES: i8 = 10; // super-sampling

pub fn main() {

    let mut canv = Canvas::new(WIDTH, HEIGHT);
    // let cam = ProjCamera::new(2.0);

    let cam = ProjCamera::new(2.0);

    let material = Rc::new(Material::Matte(Color::new(255.0, 0.0, 0.0)));
    let sphere: Sphere = Sphere::new(0.5, Vec::new([0.0, 0.0, 0.0]), Some(material));

    for y in 0..(HEIGHT as i64) {
        for x in 0..(WIDTH as i64) {
            //let r = cam.cast_ray(x as f64, y as f64);
            let r = cam.cast_ray(x as f64, y as f64);

            let mut shade_context: ShadeContext = ShadeContext::new();

            if sphere.hit(&r, 0.0, f64::MAX, &mut shade_context) {
                println!("Sphere was hit by ray: {:?}", r);
                let material = (*shade_context.material.unwrap()).clone();

                let color = match material {
                    Material::Matte(color) => color, 
                    _ => Color::new(0.0, 0.0, 0.0)
                };

                canv.set_pixel_at(x, y, color);
            }

            println!("{:?}", r);
        } 
    }

    canv.flush_ppm(String::from("image.ppm"));
    
}

