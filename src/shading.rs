use crate::color::Color;
use crate::light::PointLight;
use crate::material::ShadeContext;
use crate::ray::Ray;
use crate::vec::{Vec, Vec3, dot, reflect};


pub fn diffuse_shading(color: Color, normal: &Vec3, light: &PointLight, hit_point: &Vec3, ambient: f64, diffuse: f64) -> Color {
    let light_vec = light.pos() - *hit_point;

    
    let mut ambient_illumination = ambient;
    let mut diffuse_illumination = 0.0;

    let n_dot_l = dot(normal.normal(), light_vec.normal());

    if n_dot_l > 0.0 {
        diffuse_illumination += diffuse * n_dot_l;
    }

    let distance_attenuation = 1.0; //(2.5 / (light_vec.magnitude()));
    // println!("Attenuation: {:?}", distance_attenuation);

    // 1/(ar**2 + b*r + c)   distance attenuation

    color * light.intensity() * ( ambient_illumination + diffuse_illumination * distance_attenuation )
}

pub fn blinn_phong_shading(color: Color, normal: &Vec3, hit_point: &Vec3, light: &PointLight, r: &Ray, ambient:f64, diffuse: f64, specular: f64, shininess: f64) -> Color {

    let ambient_illumination = ambient;




    let light_vec = light.pos() - *hit_point;

    let mut diffuse_illumination = 0.0;
    
    let n_dot_l: f64 = dot(normal.normal(), light_vec.normal());
    if(n_dot_l > 0.0) {
        diffuse_illumination += diffuse * n_dot_l;
    }

    let reflect_vec = reflect(*normal, -light_vec);



    let mut specular_illumination: f64 = 0.0;

    let r_dot_v = dot(-r.dir().normal(), reflect_vec.normal());
    if(r_dot_v > 0.0) {
        specular_illumination += specular * f64::powf(r_dot_v,shininess);
    }

    color * light.intensity() * (ambient_illumination + diffuse_illumination + specular_illumination)
}

#[cfg(test)]
mod tests {
    #[test]
    fn blinn_phong_shading_test() {

    }
}