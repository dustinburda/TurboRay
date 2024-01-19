use crate::canvas::Canvas;
use crate::material::{ShadeContext, Material};
use crate::shading::diffuse_shading;
use crate::shape::Shape;
use crate::color::Color;
use crate::ray::Ray;
use crate::light::PointLight;

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

    pub fn hit(&self, r: &Ray, shade_context: &mut ShadeContext) -> bool {
        let mut hit: bool = false;

        for shape in &self.shapes{
            hit |= shape.hit(&r, 0.0, shade_context.hit_time, shade_context);
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

    let mut hit = world.hit(r, &mut shade_context);
    
    let mut color: Color = Color::new(0.0, 0.0, 0.0);

    if hit {
        
        let material = (*shade_context.material.unwrap()).clone();

        color = match material {
            // Material::Matte(color) => color, 
            Material::Matte(color, ambient, diffuse) => diffuse_shading(color, &shade_context.normal, &world.light, &shade_context.hit_point, ambient, diffuse),
            _ => Color::new(0.0, 0.0, 0.0)
        }
    } else {
        color = Color::new(0.0, 255.0 , 0.0);
    }

    color
}


