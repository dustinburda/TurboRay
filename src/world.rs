use crate::canvas::Canvas;
use crate::material::{ShadeContext, Material};
use crate::shape::Shape;
use crate::color::Color;
use crate::ray::Ray;

pub struct World {
    shapes: Vec<Box<dyn Shape>>
}

impl World {
    pub fn new() -> World {
        World {
            shapes: vec![]
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
}

pub fn trace(r: &Ray, world: &World) -> Color {
    let mut shade_context: ShadeContext = ShadeContext::new();

    let mut hit = world.hit(r, &mut shade_context);
    
    let mut color: Color = Color::new(0.0, 0.0, 0.0);

    if hit {
        
        let material = (*shade_context.material.unwrap()).clone();

        color = match material {
            Material::Matte(color) => color, 
            _ => Color::new(0.0, 0.0, 0.0)
        }
    } else {
        color = Color::new(0.0, 255.0 , 0.0);
    }

    color
}


