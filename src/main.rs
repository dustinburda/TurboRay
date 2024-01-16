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
use world::World;

use crate::camera::{OrthCamera, ProjCamera, Camera};
use crate::canvas::Canvas;
use crate::sphere::Sphere;
use crate::material::Material;
use crate::vec::Vec;
use crate::color::Color;
use std::rc::Rc;
use crate::material::ShadeContext;
use crate::world::{trace};

const WIDTH: i64 = 1920 as i64;
const HEIGHT: i64 = 1080 as i64;
const NUM_SAMPLES: i8 = 10; // super-sampling

pub fn main() {

    let mut canv = Canvas::new(WIDTH, HEIGHT);

    let cam = ProjCamera::new(2.0);

    let material1 = Rc::new(Material::Matte(Color::new(255.0, 0.0, 0.0)));
    let sphere1: Box<dyn Shape> = Box::new(Sphere::new(1.0, Vec::new([0.0, 0.0, 3.0]), Some(material1)));

    let material2 = Rc::new(Material::Matte(Color::new(0.0, 0.0, 255.0)));
    let sphere2: Box<dyn Shape> = Box::new(Sphere::new(1.0, Vec::new([1.0, 0.0, 1.0]), Some(material2)));

    let material3 = Rc::new(Material::Matte(Color::new(255.0, 0.0, 255.0)));
    let sphere3: Box<dyn Shape> = Box::new(Sphere::new(1.0, Vec::new([6.0, 0.0, 5.0]), Some(material3)));

    // let world: std::vec::Vec<Box<dyn Shape>> = vec![sphere1, sphere2, sphere3];

    let mut world = World::new();
    world.add_shape(sphere1);
    world.add_shape(sphere2);
    world.add_shape(sphere3);

    for y in 0..(HEIGHT as i64) {
        println!("Number of scanlines remaining: {:?}", HEIGHT - 1 - y);
        for x in 0..(WIDTH as i64) {

            let r = cam.cast_ray(x as f64, y as f64);
            
            canv.set_pixel_at(x, y, trace(&r, &world));
        } 
    }

    canv.flush_ppm(String::from("image.ppm"));
    
}

