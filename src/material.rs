use crate::color::Color;
use crate::vec::{Vec, Vec3};
use std::rc::Rc;
use std::option::Option;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    diffuse_color: Color,
    ambient_coeff: f64,
    diffuse_coeff: f64,
    reflective: f64,
    specular_coeff: f64,
    shininess: f64
}

impl Material {
    pub fn new(color: Color, ka: f64, kd: f64, reflective: f64 ,ks: f64, shininess: f64) -> Material {
        Material {
            diffuse_color: color,
            ambient_coeff: ka,
            diffuse_coeff: kd,
            reflective: reflective,
            specular_coeff: ks,
            shininess: shininess
        }
    }

    pub fn get_properties(&self) -> (Color, f64, f64, f64, f64, f64) {
        (self.diffuse_color.clone(), self.ambient_coeff, self.diffuse_coeff, self.reflective, self.specular_coeff, self.shininess)
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