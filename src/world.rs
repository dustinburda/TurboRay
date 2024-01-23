use crate::canvas::Canvas;
use crate::material::{ShadeContext, Material};
use crate::shading::{diffuse, specular};
use crate::shape::Shape;
use crate::color::Color;
use crate::ray::Ray;
use crate::light::{PointLight, self};

pub struct World {
    shapes: Vec<Box<dyn Shape>>,
    light: PointLight
}

impl World {
    pub fn new() -> World {
        World {
            shapes: vec![],
            light: PointLight::default()
        }
    }

    pub fn hit(&self, r: &Ray, shade_context: &mut ShadeContext, t_near: f64) -> bool {
        let mut hit: bool = false;

        for shape in &self.shapes{
            hit |= shape.hit(&r, t_near, shade_context.hit_time, shade_context);
        }

        hit
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.light = light;
    }
}

pub fn trace(r: &Ray, world: &World) -> Color {
    let mut shade_context: ShadeContext = ShadeContext::new();

    let mut hit = world.hit(r, &mut shade_context, 0.0);
    
    let mut color: Color = Color::new(0.0, 0.0, 0.0);

    if hit {
        
        let mut shadow_context: ShadeContext = ShadeContext::new();
        let shadow_ray = Ray::new(shade_context.hit_point.clone(), world.light.pos() - shade_context.hit_point);

        let in_shadow: bool = world.hit(&shadow_ray, &mut shadow_context, 0.0000000000001);
        let shadow_ray_intersection_point = shadow_context.hit_point;
        let shadowPoint_hitPoint_dist =(shadow_ray_intersection_point - shade_context.hit_point).magnitude();
        let light_hitPoint_dist = (world.light.pos() - shade_context.hit_point).magnitude();



        if in_shadow && ((shadow_ray_intersection_point - shade_context.hit_point).magnitude() < (world.light.pos() - shade_context.hit_point).magnitude()) {
            color = Color::new(0.0, 0.0, 0.0);
        } else {
            let material = (*shade_context.material.unwrap()).clone();

            color = match material {
                Material::Matte(color, ambient, diffuse) => crate::shading::diffuse(color, &shade_context.normal, &world.light, &shade_context.hit_point, ambient, diffuse),
                Material::Plastic(color, ka, kd,ks, shininess) => crate::shading::diffuse(color, &shade_context.normal, &world.light, &shade_context.hit_point, ka, kd) 
                                                                                           + crate::shading::specular(color, &shade_context.normal, &shade_context.hit_point, &world.light, &r, ks, shininess),
                _ => Color::new(1.0, 1.0, 1.0)
            }
        }    
    } else {
        color = Color::new(255.0, 51.0 , 255.0);
    }

    color
}


