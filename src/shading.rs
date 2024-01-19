use crate::color::Color;
use crate::light::PointLight;
use crate::vec::{Vec, Vec3, dot};


pub fn diffuse_shading(color: Color, normal: &Vec3, light: &PointLight, hit_point: &Vec3, ambient: f64, diffuse: f64) -> Color {
    let light_vec = light.pos() - *hit_point;


    let mut illumination = 0.0; 
    
    illumination += ambient;

    let n_dot_l = dot(*normal, light_vec);

    if n_dot_l > 0.0 {
        illumination += diffuse * n_dot_l / (normal.magnitude() * light_vec.magnitude());
    }

    (color * illumination) * light.intensity() 
}