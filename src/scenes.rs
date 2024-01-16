use crate::world::World;
use std::rc::Rc;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::vec::Vec;
use crate::color::Color;
use crate::material::Material;


pub fn scene1() -> World {
    let material1 = Rc::new(Material::Matte(Color::new(255.0, 0.0, 0.0)));
    let sphere1: Box<dyn Shape> = Box::new(Sphere::new(1.0, Vec::new([0.0, 0.0, 3.0]), Some(material1)));

    let material2 = Rc::new(Material::Matte(Color::new(0.0, 0.0, 255.0)));
    let sphere2: Box<dyn Shape> = Box::new(Sphere::new(1.0, Vec::new([1.0, 0.0, 1.0]), Some(material2)));

    let material3 = Rc::new(Material::Matte(Color::new(255.0, 0.0, 255.0)));
    let sphere3: Box<dyn Shape> = Box::new(Sphere::new(1.0, Vec::new([5.0, 0.0, 5.0]), Some(material3)));

    let mut world = World::new();
    world.add_shape(sphere1);
    world.add_shape(sphere2);
    world.add_shape(sphere3);

    world
}