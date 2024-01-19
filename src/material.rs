use crate::color::Color;
use crate::vec::{Vec, Vec3};
use std::rc::Rc;
use std::option::Option;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Matte(Color), //Diffuse
    Plastic(f64, f64), //Blinn-Phong, specular highlights
    Metal (Color), //Reflect + Tint
    Mirror, // Reflect
    Glass // Reflect + Refract
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