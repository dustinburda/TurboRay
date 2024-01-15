use crate::material::Material;
use crate::vec::{Vec3, dot};
use crate::shape::Shape;
use crate::material::ShadeContext;
use crate::ray::Ray;
use std::rc::Rc;

pub struct Sphere {
    radius: f64,
    center: Vec3,
    material: Option<Rc<Material>>
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3, material: Option<Rc<Material>>) -> Sphere {
        Sphere {
            radius: radius,
            center: center,
            material: material
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
    fn hit(&self, r: &Ray, t_near: f64, t_far: f64, shade_context: &mut ShadeContext) -> bool {
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

        shade_context.hit_time = t1;
        if t1 < t_near || t1 > t_far {
            shade_context.hit_time = t2; 
            if t2 < t_near || t2 > t_far {
                return false;
            }
        }
        
        let hit_point: Vec3 = r.at(shade_context.hit_time);
        
        shade_context.normal = self.normal_at(&hit_point);
        shade_context.material = self.material_at(&hit_point);

        true
    }

    fn normal_at(&self, hit_point: &Vec3) -> Vec3 {
        *hit_point - self.center
    }

    fn material_at(&self, hit_point: &Vec3) -> Option<Rc<Material>> {
        self.material.clone()
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
        let sphere = Sphere::new(1.0, Vec::new([0.0, 0.0, 0.0]), None);

        let r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    pub fn intersection2_test() {
        let sphere = Sphere::new(1.0, Vec::new([0.0, 0.0, 0.0]), None);

        let r = Ray::new(Vec::new([0.0, 0.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    pub fn intersection3_test() {
        let sphere = Sphere::new(1.0, Vec::new([0.0, 0.0, 0.0]), None);

        let r = Ray::new(Vec::new([0.0, 1.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    pub fn intersection4_test() {
        let sphere = Sphere::new(1.0, Vec::new([0.0, 0.0, 0.0]), None);

        let r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 
}