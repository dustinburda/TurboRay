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

    let final_color = color * light.intensity() * ( ambient_illumination + diffuse_illumination * distance_attenuation );
    // println!("R: {:?}, G: {:?}, B: {:?}", final_color.r(), final_color.g(), final_color.b());
    final_color
}

pub fn blinn_phong_shading(color: Color, normal: &Vec3, hit_point: &Vec3, light: &PointLight, r: &Ray, ambient:f64, diffuse: f64, specular: f64, shininess: f64) -> Color {

    // println!("IN BLINN PHONG SHADING!");
    let ambient_illumination = ambient;

    let light_vec = light.pos() - *hit_point;

    let mut diffuse_illumination = 0.0;
    
    let n_dot_l: f64 = dot(normal.normal(), light_vec.normal());
    if(n_dot_l > 0.0) {
        diffuse_illumination += diffuse * n_dot_l;
    }

    let reflect_vec = reflect(*normal, -light_vec);



    let mut specular_illumination: f64 = 0.0;

    let r_dot_v = dot((-r.dir()).normal(), reflect_vec.normal());
    if(r_dot_v > 0.0) {
        // println!("r_dot_v: {:?}", r_dot_v);
        specular_illumination += specular * f64::powf(r_dot_v,shininess);
        if (specular_illumination > 0.5) {
            println!("Specular: {:?}", specular_illumination);
        }
    }
    // println!("R: {:?}, G: {:?}, B: {:?}", color.r(), color.g(), color.b());
    let final_color = color * light.intensity() * (ambient_illumination + diffuse_illumination + specular_illumination);
    // println!("R: {:?}, G: {:?}, B: {:?}", final_color.r(), final_color.g(), final_color.b());
    final_color
}

#[cfg(test)]
mod tests {
    use crate::light::PointLight;
    use crate::shading::blinn_phong_shading;
    use crate::color::Color;
    use crate::ray::Ray;
    use crate::vec::{Vec, Vec3};

    #[test]
    fn blinn_phong_shading_test() {
        let light1 = PointLight::new(1.0, Vec::new([0.0, 0.0, -10.0]));
        let ray1 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let color1 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
                                        &Vec::new([0.0, 0.0, -1.0]), 
                                     &Vec::new([0.0, 0.0, 0.0]), 
                                               &light1, &ray1, 0.1, 0.9, 0.9, 200.0);

        assert_eq!(color1, Color::new(255.0 * 1.9, 255.0 * 1.9, 255.0 * 1.9));


        let light2 = PointLight::new(1.0, Vec::new([0.0, 0.0, -10.0]));
        let ray2 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0]));
        let color2 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
                                        &Vec::new([0.0, 0.0, -1.0]), 
                                     &Vec::new([0.0, 0.0, 0.0]), 
                                               &light2, &ray2, 0.1, 0.9, 0.9, 200.0);

        assert_eq!(color2, Color::new(255.0, 255.0, 255.0));


        let light3 = PointLight::new(1.0, Vec::new([0.0, 10.0, -10.0]));
        let ray3 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let color3 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
                                        &Vec::new([0.0, 0.0, -1.0]), 
                                     &Vec::new([0.0, 0.0, 0.0]), 
                                               &light3, &ray3, 0.1, 0.9, 0.9, 200.0);

        assert_eq!(color3, Color::new(187.781, 187.781, 187.781));


        let light4 = PointLight::new(1.0, Vec::new([0.0, 10.0, -10.0]));
        let ray4 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0]));
        let color4 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
                                        &Vec::new([0.0, 0.0, -1.0]), 
                                     &Vec::new([0.0, 0.0, 0.0]), 
                                               &light4, &ray4, 0.1, 0.9, 0.9, 200.0);

        assert_eq!(color4, Color::new(417.281, 417.281, 417.281));


        let light5 = PointLight::new(1.0, Vec::new([0.0, 0.0, 10.0]));
        let ray5 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let color5 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
                                        &Vec::new([0.0, 0.0, -1.0]), 
                                     &Vec::new([0.0, 0.0, 0.0]), 
                                               &light5, &ray5, 0.1, 0.9, 0.9, 200.0);

        assert_eq!(color5, Color::new(25.5, 25.5, 25.5));
    }
}