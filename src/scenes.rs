use crate::transformations::{translation, scale};
use crate::world::World;
use std::rc::Rc;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::vec::Vec;
use crate::color::Color;
use crate::material::{Material, self};
use crate::light::PointLight;
use crate::plane::{Plane, self};


pub fn scene1() -> World {
    // let material1 = Rc::new(Material::Matte(Color::new(100.0, 51.0, 255.0), 0.1, 0.9));
    // let material1 = Rc::new(Material::Mirror(Color::new(255.0, 255.0, 255.05), 0.0, 0.0,  0.85, 0.95, 300.0));
    let material1 = Rc::new(Material::new(Color::new(255.0, 255.0, 255.05), 0.0, 0.0,  0.85, 0.95, 300.0));
    let mut sphere1: Box<dyn Shape> = Box::new(Sphere::new(1.0, Some(material1)));

    sphere1.set_transform(translation(1.0, 1.8, 7.0) * scale(1.0, 1.0, 1.0));

    //let material2 = Rc::new(Material::Matte(Color::new(255.0, 51.0, 255.0), 0.1, 0.9));
    // let material2 = Rc::new(Material::new(Color::new(255.0, 0.0, 0.0), 0.1, 0.9, 0.0, 0.4, 10.0));
    // let material2 = Rc::new(Material::Mirror(Color::new(255.0, 255.0, 255.0), 0.0, 0.0,  0.85, 0.95, 300.0));
    let material2 = Rc::new(Material::new(Color::new(255.0, 255.0, 255.0), 0.0, 0.0,  0.85, 0.95, 300.0));
    let mut sphere2: Box<dyn Shape> = Box::new(Sphere::new(1.0, Some(material2)));
    sphere2.set_transform(translation(-1.0, 0.0, 5.0) * scale(1.0, 1.0, 1.0));

    // let material3 = Rc::new(Material::Matte(Color::new(255.0, 51.0, 251.0), 0.1, 0.9));
    // let material3 = Rc::new(Material::Plastic(Color::new(255.0, 51.0, 255.0), 0.1, 0.9, 0.0, 0.4, 10.0));
    let material3 = Rc::new(Material::new(Color::new(255.0, 51.0, 255.0), 0.1, 0.9, 0.0, 0.4, 10.0));
    let mut sphere3: Box<dyn Shape> = Box::new(Sphere::new(1.0, Some(material3)));
    sphere3.set_transform(translation(7.0, 0.0, 17.0));


    // let material4 = Rc::new(Material::Matte(Color::new(0.0, 0.0, 255.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let material4 = Rc::new(Material::new(Color::new(0.0, 0.0, 255.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let mut plane1: Box<dyn Shape> = Box::new(Plane::new(Vec::new([0.0, -1.0, 0.0]), Vec::new([0.0, 1.0, 0.0]), Some(material4)));

    // let material5 = Rc::new(Material::Matte(Color::new(255.0, 0.0, 0.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let material5 = Rc::new(Material::new(Color::new(255.0, 0.0, 0.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let mut plane2: Box<dyn Shape> = Box::new(Plane::new(Vec::new([-4.0, 0.0, 0.0]), Vec::new([1.0, 0.0, 0.0]), Some(material5)));

    // let material6 = Rc::new(Material::Matte(Color::new(125.0, 125.0, 125.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let material6 = Rc::new(Material::new(Color::new(125.0, 125.0, 125.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let mut plane3: Box<dyn Shape> = Box::new(Plane::new(Vec::new([0.0, 0.0, 20.0]), Vec::new([0.0, 0.0, -1.0]), Some(material6)));

    // let material7 = Rc::new(Material::Matte(Color::new(125.0, 125.0, 125.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let material7 = Rc::new(Material::new(Color::new(125.0, 125.0, 125.0), 0.18, 0.9, 0.0, 0.0 , 0.0));
    let mut plane4: Box<dyn Shape> = Box::new(Plane::new(Vec::new([0.0, 0.0, -20.0]), Vec::new([0.0, 0.0, -1.0]), Some(material7)));


    let light: PointLight = PointLight::new(1.0, Vec::new([5.0, 10.0, -10.0]));

    let mut world = World::new();

    world.add_shape(sphere1);
    world.add_shape(sphere2);
     world.add_shape(sphere3);
    world.add_shape(plane1);
    world.add_shape(plane2);
    world.add_shape(plane3);
    world.add_shape(plane4);

    world.add_light(light);

    world
}