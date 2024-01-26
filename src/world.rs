use crate::canvas::Canvas;
use crate::material::{ShadeContext, Material};
use crate::shading::{diffuse, specular, shadow, reflection};
use crate::shape::Shape;
use crate::color::{Color, BLACK, WHITE};
use crate::ray::{Ray, self};
use crate::light::{PointLight, self};

pub struct World {
    shapes: Vec<Box<dyn Shape>>,
    lights: Vec<PointLight>
}

impl World {
    pub fn new() -> World {
        World {
            shapes: vec![],
            lights: vec![]
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
        self.lights.push(light);
    }
}

pub fn trace(r: &Ray, world: &World, max_depth: i8, epsilon: f64) -> Color {

    if(max_depth == 0) {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut shade_context: ShadeContext = ShadeContext::new();

    let mut hit = world.hit(r, &mut shade_context, epsilon);
    
    let mut color: Color = Color::new(0.0, 0.0, 0.0);

    if hit {
        let material = (*shade_context.material.unwrap()).clone();
        let (diffuse_color, ka, kd, reflective, ks, shininess) = material.get_properties();
        let mut diffuse_illumination: f64 = 0.0;
        let mut specular_illumination: f64 = 0.0;
        
        // if(world.lights.len() <= 1) {
            diffuse_illumination += ka; // should only be added if there is only one light
        // }

        for light in &world.lights {
            let in_shadow = shadow(&shade_context.hit_point, light, &world);

            if in_shadow {
               continue;
            } else {
                if kd > 0.0 {
                    diffuse_illumination += crate::shading::diffuse( &shade_context.normal, light, &shade_context.hit_point, kd);
                }
                if ks > 0.0 {
                    specular_illumination +=  crate::shading::specular( &shade_context.normal, &shade_context.hit_point, light, &r, ks, shininess);
                }
            }
            // ==== LOOP
        }       

        color += diffuse_color * diffuse_illumination + WHITE * specular_illumination;
        if reflective > 0.0 {
            color += reflection(r, reflective, &shade_context.hit_point, &shade_context.normal, &world, max_depth -1);
        }    
        //=======================================
    } else {
        color = BLACK;
    }

    color
}


