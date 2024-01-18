use crate::material::Material;
use crate::vec::{Vec3, dot, Vec};
use crate::shape::Shape;
use crate::material::ShadeContext;
use crate::matrix::{Matrix, Mat44};
use crate::ray::Ray;
use std::rc::Rc;

pub struct Sphere {
    radius: f64,
    center: Vec3,
    material: Option<Rc<Material>>,
    obj_to_world: Mat44,
    world_to_obj: Mat44
}

impl Sphere {
    pub fn new(radius: f64, material: Option<Rc<Material>>) -> Sphere {
        Sphere {
            radius: radius,
            center: Vec::new([0.0, 0.0, 0.0]),
            material: material,
            obj_to_world: Matrix::identity(),
            world_to_obj: Matrix::identity()
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_near: f64, t_far: f64, shade_context: &mut ShadeContext) -> bool {

        let r = ray.transform(&self.world_to_obj);

        let a:f64 = dot(r.dir(), r.dir());
        let b: f64 = 2.0 * dot(r.dir(), r.orig() - self.center);
        let c: f64 = dot(r.orig() - self.center, r.orig()- self.center) - self.radius * self.radius;

        let discriminant: f64 = b * b - 4.0 * a * c;
        
        // TODO: use float_equal
        if discriminant < 0.0 {
            return false;
        }

        let t1: f64 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2: f64 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

        let mut hit_time: f64 = 0.0;

        hit_time = t1;
        if t1 < t_near || t1 > t_far {
            hit_time = t2; 
            if t2 < t_near || t2 > t_far {
                return false;
            }
        }
        
        let hit_point: Vec3 = r.at(hit_time);

        shade_context.hit_time = hit_time;
        shade_context.normal = ((self.world_to_obj).t() * self.normal_at(&hit_point).homogenize_vec()).dehomogenize();
        shade_context.material = self.material_at(&hit_point);

        true
    }

    // TODO: flip normal if dot product with incident ray is positive
    fn normal_at(&self, hit_point: &Vec3) -> Vec3 {
        *hit_point - self.center
    }

    fn material_at(&self, hit_point: &Vec3) -> Option<Rc<Material>> {
        self.material.clone()
    }

    fn set_transform(&mut self, mat: Mat44) {
        self.obj_to_world = mat;
        self.world_to_obj = mat.inv();
    }
}


#[cfg(test)]
mod tests {

    use crate::material::ShadeContext;
    use crate::shape::Shape;
    use crate::sphere::Sphere;
    use crate::vec::Vec;
    use crate::ray::Ray;

    #[test]
    pub fn intersection1_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    pub fn intersection2_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    pub fn intersection3_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 1.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    pub fn intersection4_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 
}