use crate::transformations::{translation, scale};
use crate::world::World;
use std::rc::Rc;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::vec::Vec;
use crate::color::Color;
use crate::material::Material;
use crate::light::PointLight;


pub fn scene1() -> World {
    let material1 = Rc::new(Material::Matte(Color::new(255.0, 51.0, 255.0), 0.1, 0.9));
    let mut sphere1: Box<dyn Shape> = Box::new(Sphere::new(1.0, Some(material1)));

    sphere1.set_transform(translation(5.0, 0.0, 5.0) * scale(1.0, 1.0, 1.0));

    let material2 = Rc::new(Material::Matte(Color::new(0.0, 0.0, 255.0), 0.1, 0.9));
    let mut sphere2: Box<dyn Shape> = Box::new(Sphere::new(1.0, Some(material2)));
    sphere2.set_transform(translation(1.0, 0.0, 1.0) * scale(1.0, 1.0, 1.0));

    let material3 = Rc::new(Material::Matte(Color::new(255.0, 0.0, 0.0), 0.1, 0.9));
    let mut sphere3: Box<dyn Shape> = Box::new(Sphere::new(1.0, Some(material3)));
    sphere3.set_transform(translation(0.0, 0.0, 3.0));

    let light: PointLight = PointLight::new(1.0, Vec::new([-6.0, 0.0, 0.0]));

    let mut world = World::new();
    world.add_shape(sphere1);
    world.add_shape(sphere2);
    world.add_shape(sphere3);

    world.add_light(light);

    world
}