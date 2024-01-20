use crate::{ray::Ray, vec::{Vec3, Vec}, WIDTH, HEIGHT};
use rand::Rng;


pub enum AliasMode {
    AntiAliasOn,
    AntiAliasOff
}

pub trait Camera {
    fn cast_ray(&self, x: f64, y: f64, mode: AliasMode) -> Ray;
}

pub struct ProjCamera {
    focal_distance: f64,

    viewport_u: Vec3,
    viewport_v: Vec3 
}

impl ProjCamera {
    pub fn new(focal_distance: f64) -> ProjCamera {
        let aspect_ratio = (WIDTH as f64) / (HEIGHT as f64);

        ProjCamera {
            focal_distance: focal_distance,

            viewport_u: Vec::new([7.0 * aspect_ratio / (WIDTH as f64), 0.0, 0.0]), //TODO: CHANGE TO x= 2.0. z =0.0
            viewport_v: Vec::new([0.0, - 7.0 / (HEIGHT as f64), 0.0]) // TODO  CHANGE TO x= -2.0. z =0.0
        }
    }

    pub fn look_at() -> ProjCamera {
        todo!()
    }
}

impl Camera for ProjCamera {
    fn cast_ray(&self, x: f64, y: f64, mode: AliasMode) -> Ray {
        let aspect_ratio = (WIDTH as f64) / (HEIGHT as f64);

        let orig = Vec::new([0.0, 0.0, -self.focal_distance]);
        
        let upper_left_corner = Vec::new([-3.5 * aspect_ratio, 3.5 , 0.0]); // TODO: change to x=-1.0, y=1.0, z=0.0
        let dir =  match mode {
            AliasMode::AntiAliasOff => upper_left_corner + (0.5 * self.viewport_u + 0.5 * self.viewport_v) + x * self.viewport_u + y * self.viewport_v - orig,
            AliasMode::AntiAliasOn => upper_left_corner 
                                        + ((rand::thread_rng().gen_range(0.0..1.0)) * self.viewport_u + (rand::thread_rng().gen_range(0.0..1.0)) * self.viewport_v) 
                                        + x * self.viewport_u + y * self.viewport_v - orig
        };
    
        Ray::new(orig, dir)
    }
}

pub struct OrthCamera {
    viewport_u: Vec3,
    viewport_v: Vec3 
}

impl OrthCamera {
    pub fn new() -> OrthCamera {
        let aspect_ratio: f64 = WIDTH as f64 / HEIGHT as f64;

        OrthCamera {
            

            //TODO 
            viewport_u: Vec::new([2.0 * aspect_ratio / (WIDTH as f64), 0.0, 0.0]),
            viewport_v: Vec::new([0.0, - 2.0 / (HEIGHT as f64), 0.0])
        }
    }


    pub fn look_at() -> OrthCamera {
        todo!()
    }
}

impl Camera for OrthCamera {
    fn cast_ray(&self, x: f64, y: f64, mode: AliasMode) -> Ray {
        let aspect_ratio: f64 = WIDTH as f64 / HEIGHT as f64;

        let upper_left_corner = Vec::new([-1.0 * aspect_ratio, 1.0, 0.0]);

        let orig = upper_left_corner + 0.5  * self.viewport_u + 0.5 * self.viewport_v + x * self.viewport_u + y * self.viewport_v;

        Ray::new(orig, Vec::new([0.0, 0.0, 1.0]))
    }
}