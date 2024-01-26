use crate::color::Color;
use crate::vec::{Vec, Vec3};
use std::rc::Rc;
use std::option::Option;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Matte(Color, f64, f64, f64, f64, f64),  //diffuse color, ambient coeff, diffuse coeff, reflective, specular coeff, shininess
    Plastic(Color, f64, f64, f64, f64, f64),  //diffuse color, ambient coeff, diffuse coeff,           , specular coeff, shininess
    Mirror (Color, f64, f64, f64, f64, f64),  //diffuse color, ambient coeff, diffuse coeff, reflective, specular coeff, shininess
    //diffuse color, ambient coeff, diffuse coeff, specular coeff, shininess, reflective
}
impl Material {
    pub fn default_matte() -> Material {
        Material::Matte(Color::new(255.0, 255.0, 255.0), 0.1, 0.9, 0.0, 0.0, 0.0)
    }

    pub fn default_plastic() -> Material{
        Material::Plastic(Color::new(255.0, 255.0, 255.0), 0.1, 0.9, 0.0, 0.9, 200.0)
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