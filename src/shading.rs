use crate::color::Color;
use crate::light::PointLight;
use crate::material::ShadeContext;
use crate::ray::Ray;
use crate::vec::{Vec, Vec3, dot, reflect};
use crate::world::{World, trace};


pub fn diffuse(normal: &Vec3, light: &PointLight, hit_point: &Vec3, diffuse: f64) -> f64 {
    let light_vec = light.pos() - *hit_point;


    let mut diffuse_illumination = 0.0;

    let n_dot_l = dot(normal.normal(), light_vec.normal());

    if n_dot_l > 0.0 {
        diffuse_illumination += diffuse * n_dot_l;
    }

    let distance_attenuation = 1.0; //(2.5 / (light_vec.magnitude()));

    diffuse_illumination = light.diffuse_intensity() * diffuse_illumination;

    diffuse_illumination
}

pub fn specular(normal: &Vec3, hit_point: &Vec3, light: &PointLight, r: &Ray, specular: f64, shininess: f64) ->  f64 {
    let light_vec = light.pos() - *hit_point;

    let mut specular_illumination: f64 = 0.0;
    
    let n_dot_l: f64 = dot(normal.normal(), light_vec.normal());
    if(n_dot_l > 0.0) {
        let reflect_vec = reflect(*normal, -light_vec.normal());
        let r_dot_v = dot((-r.dir()).normal(), reflect_vec.normal());

        if(r_dot_v > 0.0) {
            specular_illumination += specular * f64::powf(r_dot_v,shininess);
        }
    }


    specular_illumination = light.specular_intensity() * specular_illumination;

    specular_illumination
}

pub fn shadow(hit_point: &Vec3, light: &PointLight, world: &World) -> bool {
        // TODO: refactor here
        let mut shadow_context: ShadeContext = ShadeContext::new();
        let shadow_ray = Ray::new(hit_point.clone(), light.pos() - *hit_point);

        let in_shadow: bool = world.hit(&shadow_ray, &mut shadow_context, 0.0000000000001);
        let shadow_ray_intersection_point = shadow_context.hit_point;
        
        let shadowPoint_hitPoint_dist =(shadow_ray_intersection_point - *hit_point).magnitude();
        let light_hitPoint_dist = (light.pos() - *hit_point).magnitude();

        //in_shadow && ((shadow_ray_intersection_point - *hit_point).magnitude() < (light.pos() - *hit_point).magnitude())
        in_shadow && (shadowPoint_hitPoint_dist < light_hitPoint_dist)
    }

pub fn reflection(r: &Ray, reflective: f64, hit_point: &Vec3, normal: &Vec3, world: &World, max_depth: i8) -> Color {
    let mut normal_copy = (*normal).clone();

    if (dot(r.dir(), *normal) >= 0.0) {
        normal_copy *= -1.0;
    }
   
    
    let reflected_dir = reflect(normal_copy, r.dir().normal());

    let reflected_ray = Ray::new(*hit_point, reflected_dir);

    reflective * trace(&reflected_ray, world, max_depth, 0.001)
}

pub fn refraction() -> Color {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::light::{PointLight, self};
    use crate::shading::{diffuse, specular};
    use crate::color::Color;
    use crate::ray::Ray;
    use crate::vec::{Vec, Vec3};

    #[test]
    fn blinn_phong_shading_test() {
        let light1 = PointLight::new(1.0, 1.0, Vec::new([0.0, 0.0, -10.0]));
        let ray1 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let obj_color1 = Color::new(255.0, 255.0, 255.0);
        let normal1 = Vec::new([0.0, 0.0, -1.0]);
        let hitpoint1 = Vec::new([0.0, 0.0, 0.0]);
        let color1 = obj_color1 * 0.1 + obj_color1 * diffuse( &normal1, &light1, &hitpoint1, 0.9) +
                            Color::new(255.0, 255.0, 255.0) * specular(&normal1, &hitpoint1,&light1, &ray1, 0.9, 200.0);

        assert_eq!(color1, Color::new(255.0 * 1.9, 255.0 * 1.9, 255.0 * 1.9));


        let light2 = PointLight::new(1.0, 1.0, Vec::new([0.0, 0.0, -10.0]));
        let ray2 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0]));
        let color2 =  obj_color1 * 0.1 +  obj_color1 * diffuse( &normal1, &light2, &hitpoint1, 0.9) +
                             Color::new(255.0, 255.0, 255.0) * specular(&normal1, &hitpoint1,&light2, &ray2, 0.9, 200.0);

        assert_eq!(color2, Color::new(255.0, 255.0, 255.0));


        let light3 = PointLight::new(1.0, 1.0, Vec::new([0.0, 10.0, -10.0]));
        let ray3 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let color3 =  obj_color1 * 0.1  + obj_color1 *  diffuse( &normal1, &light3, &hitpoint1, 0.9,) +
                    Color::new(255.0, 255.0, 255.0) * specular(&normal1,&hitpoint1,&light3, &ray3, 0.9, 200.0);

        assert_eq!(color3, Color::new(187.781, 187.781, 187.781));


        let light4 = PointLight::new(1.0, 1.0, Vec::new([0.0, 10.0, -10.0]));
        let ray4 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0]));
        let color4 = obj_color1 * 0.1 +  obj_color1 * diffuse( &normal1, &light4, &hitpoint1,  0.9) +
                        Color::new(255.0, 255.0, 255.0) * specular(&normal1, &hitpoint1, &light4, &ray4, 0.9, 200.0);

        assert_eq!(color4, Color::new(417.281, 417.281, 417.281));


        let light5 = PointLight::new(1.0, 1.0,Vec::new([0.0, 0.0, 10.0]));
        let ray5 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let color5 = obj_color1 * 0.1 + obj_color1 * diffuse( &normal1, &light5, &hitpoint1, 0.9) +
                            Color::new(255.0, 255.0, 255.0) *  specular(&normal1, &hitpoint1, &light5, &ray5, 0.9, 200.0);

        assert_eq!(color5, Color::new(25.5, 25.5, 25.5));
    }
}