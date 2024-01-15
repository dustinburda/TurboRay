use crate::material::{Material, ShadeContext};
use crate::vec::Vec3;
use std::rc::Rc;
use crate::ray::Ray;

pub trait Shape {
    fn hit(&self, r: &Ray, t_near: f64, t_far: f64, shade_context: &mut ShadeContext) -> bool;
    fn normal_at(&self, hit_point: &Vec3) -> Vec3;
    fn material_at(&self, hit_point: &Vec3) -> Option<Rc<Material>>;
}

