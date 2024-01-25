use crate::color::Color;
use crate::vec::{Vec, Vec3};
use std::rc::Rc;
use std::option::Option;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Matte(Color, f64, f64), //Diffuse
    Plastic(Color, f64, f64, f64, f64), //diffuse color, ambient coeff, diffuse,specular, shininess
    Metal (Color), //Reflect + Tint
    Mirror (Color, f64, f64, f64, f64), // diffuse color, diffuse coeff, reflective, specular, shininess
    Glass, // Reflect + Refract
    PartiallyReflective(Color, f64)
}
impl Material {
    pub fn default_matte() -> Material {
        Material::Matte(Color::new(255.0, 255.0, 255.0), 0.1, 0.9)
    }

    pub fn default_plastic() -> Material{
        Material::Plastic(Color::new(255.0, 255.0, 255.0), 0.1, 0.9, 0.9, 200.0)
    }
}

pub struct ShadeContext {
    pub normal: Vec3,
    pub material: Option<Rc<Material>>,
    pub hit_time: f64,
    pub hit_point: Vec3
    // TODO: add a hitpoint for shadows
}

impl ShadeContext {
    pub fn new() -> Self {
        ShadeContext {
            normal: Vec::new([0.0, 0.0, 0.0]),
            material: None,
            hit_time: f64::MAX,
            hit_point: Vec::new([0.0, 0.0, 0.0])
        }
    }
}