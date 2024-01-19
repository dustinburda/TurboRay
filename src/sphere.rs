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
        
        let world_point: Vec3 = ray.at(hit_time);
        let object_point = (self.world_to_obj * world_point.homogenize()).dehomogenize();

        assert_eq!(object_point, r.at(hit_time));

        shade_context.hit_time = hit_time;
        // let mut normal = ((self.world_to_obj).t() * self.normal_at(&object_point).homogenize_vec()).dehomogenize_normal().normal();
        // shade_context.normal = ((self.world_to_obj).t() * self.normal_at(&object_point).homogenize_vec()).dehomogenize_normal().normal();shade_context.normal = ((self.world_to_obj).t() * self.normal_at(&object_point).homogenize_vec()).dehomogenize_normal().normal();
        // normal[3] = 0.0;
        // shade_context.normal = normal.dehomogenize().normal();

        shade_context.normal = self.normal_at(&world_point);



        shade_context.material = self.material_at(&object_point);
        shade_context.hit_point = world_point;

        true
    }

    // TODO: flip normal if dot product with incident ray is positive
    fn normal_at(&self, world_hit_point: &Vec3) -> Vec3 {
       // ( *hit_point - self.center).normal()
       let object_point = (self.world_to_obj * world_hit_point.homogenize()).dehomogenize();
       let object_normal = object_point - self.center;
       let world_normal = (self.world_to_obj.t() * object_normal.homogenize_vec()).dehomogenize_normal();

       world_normal.normal()
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
    use crate::transformations::{scale, translation, rotz};
    use crate::vec::Vec;
    use crate::ray::Ray;

    #[test]
    fn intersection1_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    fn intersection2_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    fn intersection3_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 1.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 


    #[test]
    fn intersection4_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    } 

    #[test]
    fn intersection5_test() {
        let sphere = Sphere::new(1.0, None);

        let r = Ray::new(Vec::new([0.0, 0.0, 5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(!sphere.hit(&r, 0.0, f64::MAX , &mut shade_context))
    }

    #[test]
    fn intersection_transform_scale_test() {
        let mut sphere = Sphere::new(1.0, None);
        sphere.set_transform(scale(2.0, 2.0, 2.0));

        let r = Ray::new(Vec::new([0.0, 0.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(sphere.hit(&r, 0.0, f64::MAX, &mut shade_context))
    }


    #[test]
    fn intersection_transform_translate_test() {
        let mut sphere = Sphere::new(1.0, None);
        sphere.set_transform(translation(5.0, 0.0, 0.0));

        let r = Ray::new(Vec::new([0.0, 0.0, -5.0]), Vec::new([0.0, 0.0, 1.0]));

        let mut shade_context = ShadeContext::new(); 

        assert!(!sphere.hit(&r, 0.0, f64::MAX, &mut shade_context))
    }

    #[test]
    fn normal_at_test() {
        let s: Sphere = Sphere::new(1.0, None);

        let n1 = s.normal_at(&Vec::new([1.0, 0.0, 0.0]));
        assert_eq!(n1, Vec::new([1.0, 0.0, 0.0]));

        let n2 = s.normal_at(&Vec::new([0.0, 1.0, 0.0]));
        assert_eq!(n2, Vec::new([0.0, 1.0, 0.0]));

        let n3 = s.normal_at(&Vec::new([0.0, 0.0, 1.0]));
        assert_eq!(n3, Vec::new([0.0, 0.0, 1.0]));


        let n4 = s.normal_at(&Vec::new([f64::sqrt(3.0) / 3.0, 
                                                                    f64::sqrt(3.0) / 3.0, 
                                                                    f64::sqrt(3.0) / 3.0]));
        assert_eq!(n4, Vec::new([f64::sqrt(3.0) / 3.0, f64::sqrt(3.0) / 3.0, f64::sqrt(3.0) / 3.0]));
    }

    #[test]
    fn normal_at_transformed_test() {
        let mut s1: Sphere = Sphere::new(1.0, None);
        s1.set_transform(translation(0.0, 1.0, 0.0));

        let normal_s1 = s1.normal_at(&Vec::new([0.0, 1.70711, -0.70711]));
        assert_eq!(normal_s1, Vec::new([0.0, 0.70711, -0.70711]));



        let mut s2: Sphere = Sphere::new(1.0, None);
        s2.set_transform(scale(1.0, 0.5, 1.0) * rotz(std::f64::consts::PI / 5.0));

        let normal_s2 = s2.normal_at(&Vec::new([0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0]));
        assert_eq!(normal_s2, Vec::new([0.0, 0.97014, -0.24254]));


    }
}