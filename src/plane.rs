use crate::matrix::{Mat44, Matrix};
use crate::vec::{Vec, Vec3, dot};
use crate::shape::{Shape};
use crate::material::ShadeContext;
use crate::material::Material;
use crate::ray::Ray;
use std::rc::Rc;

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Option<Rc<Material>>,
    obj_to_world: Mat44,
    world_to_obj: Mat44
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, mat: Option<Rc<Material>>) -> Plane {
        Plane {
            point: point,
            normal: normal,
            material: mat,
            obj_to_world: Matrix::identity(),
            world_to_obj: Matrix::identity()
        }
    }
}

impl Shape for Plane {
    fn set_transform(&mut self, mat: Mat44) {
        self.obj_to_world = mat;
        self.world_to_obj = mat.inv();
    }

    fn hit(&self, ray: &Ray, t_near: f64, t_far: f64, shade_context: &mut ShadeContext) -> bool {
        let r = ray.transform(&self.world_to_obj);

        let t = dot(self.normal, self.point - r.orig()) / dot(self.normal, r.dir());

        if t < t_near || t > t_far {
            return false;
        }

        let world_hit_point = ray.at(t);

        shade_context.hit_time = t;
        shade_context.normal = self.normal_at(&world_hit_point);
        shade_context.material = self.material.clone();
        shade_context.hit_point = world_hit_point;

        true
    }

    fn material_at(&self, hit_point: &Vec3) -> Option<Rc<Material>> {
        self.material.clone()
    }

    fn normal_at(&self, hit_point: &Vec3) -> Vec3 {
        (self.world_to_obj.t() * self.normal.homogenize_vec()).dehomogenize_normal()
    }
}