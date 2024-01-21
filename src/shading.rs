use crate::color::Color;
use crate::light::PointLight;
use crate::material::ShadeContext;
use crate::ray::Ray;
use crate::vec::{Vec, Vec3, dot, reflect};


pub fn diffuse(color: Color, normal: &Vec3, light: &PointLight, hit_point: &Vec3, ambient: f64, diffuse: f64) -> Color {
    let light_vec = light.pos() - *hit_point;

    
    let mut ambient_illumination = ambient;
    let mut diffuse_illumination = 0.0;

    let n_dot_l = dot(normal.normal(), light_vec.normal());

    if n_dot_l > 0.0 {
        diffuse_illumination += diffuse * n_dot_l;
    }

    let distance_attenuation = 1.0; //(2.5 / (light_vec.magnitude()));

    let final_color = color * light.intensity() * ( ambient_illumination + diffuse_illumination );

    final_color
}

pub fn specular(color: Color, normal: &Vec3, hit_point: &Vec3, light: &PointLight, r: &Ray, specular: f64, shininess: f64) -> Color {
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


    let final_color = Color::new(255.0, 255.0, 255.0) * light.intensity() * specular_illumination;
    
    final_color
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
        let light1 = PointLight::new(1.0, Vec::new([0.0, 0.0, -10.0]));
        let ray1 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let obj_color1 = Color::new(255.0, 255.0, 255.0);
        let normal1 = Vec::new([0.0, 0.0, -1.0]);
        let hitpoint1 = Vec::new([0.0, 0.0, 0.0]);
        let color1 = diffuse(obj_color1, &normal1, &light1, &hitpoint1, 0.1, 0.9) +
                        specular(obj_color1,&normal1, &hitpoint1,&light1, &ray1, 0.9, 200.0);

        assert_eq!(color1, Color::new(255.0 * 1.9, 255.0 * 1.9, 255.0 * 1.9));


        let light2 = PointLight::new(1.0, Vec::new([0.0, 0.0, -10.0]));
        let ray2 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0]));
        let color2 =    diffuse(obj_color1, &normal1, &light2, &hitpoint1, 0.1, 0.9) +
                                specular(obj_color1, &normal1, &hitpoint1,&light2, &ray2, 0.9, 200.0);

        assert_eq!(color2, Color::new(255.0, 255.0, 255.0));


        let light3 = PointLight::new(1.0, Vec::new([0.0, 10.0, -10.0]));
        let ray3 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        let color3 =    diffuse(obj_color1, &normal1, &light3, &hitpoint1, 0.1, 0.9,) +
                                specular(obj_color1,&normal1,&hitpoint1,&light3, &ray3, 0.9, 200.0);

        assert_eq!(color3, Color::new(187.781, 187.781, 187.781));


        // let light4 = PointLight::new(1.0, Vec::new([0.0, 10.0, -10.0]));
        // let ray4 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0]));
        // let color4 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
        //                                 &Vec::new([0.0, 0.0, -1.0]), 
        //                              &Vec::new([0.0, 0.0, 0.0]), 
        //                                        &light4, &ray4, 0.1, 0.9, 0.9, 200.0);

        // assert_eq!(color4, Color::new(417.281, 417.281, 417.281));


        // let light5 = PointLight::new(1.0, Vec::new([0.0, 0.0, 10.0]));
        // let ray5 = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 1.0]));
        // let color5 = blinn_phong_shading(Color::new(255.0, 255.0, 255.0), 
        //                                 &Vec::new([0.0, 0.0, -1.0]), 
        //                              &Vec::new([0.0, 0.0, 0.0]), 
        //                                        &light5, &ray5, 0.1, 0.9, 0.9, 200.0);

        // assert_eq!(color5, Color::new(25.5, 25.5, 25.5));
    }
}