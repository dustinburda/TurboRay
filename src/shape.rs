use crate::material::Material;
use crate::vec::Vec3;
use crate::color::Color;
use std::rc::Rc;
use crate::ray::Ray;

pub trait Shape {
    fn hit(&self, r: &Ray, t_near: &mut f64, t_far: &mut f64, shade_context: &mut ShadeContext) -> bool;
    fn normal_at(&self, hit_point: &Vec3) -> Vec3;
    fn material_at(&self, hit_point: &Vec3) -> Rc<Material>;
}

pub struct ShadeContext {
    pub normal: Vec3,
    pub material: Rc<Material>,
    pub hit_time: f64
}
