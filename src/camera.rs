use crate::{ray::Ray, vec::{Vec3, Vec}, WIDTH, HEIGHT};

pub trait Camera {
    fn cast_ray(&self, x: f64, y: f64) -> Ray;
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

            viewport_u: Vec::new([2.0 * aspect_ratio / (WIDTH as f64), 0.0, 0.0]),
            viewport_v: Vec::new([0.0, - 2.0 / (HEIGHT as f64), 0.0])
        }
    }

    pub fn look_at() -> ProjCamera {
        todo!()
    }
}

impl Camera for ProjCamera {
    fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let aspect_ratio = (WIDTH as f64) / (HEIGHT as f64);

        let orig = Vec::new([0.0, 0.0, -self.focal_distance]);
        
        let upper_left_corner = Vec::new([-1.0 * aspect_ratio, 1.0 , 0.0]);
        let dir = upper_left_corner + (0.5 * self.viewport_u + 0.5 * self.viewport_v) + x * self.viewport_u + y * self.viewport_v;

        Ray::new(orig, dir)
    }
}

struct OrthCamera {
    viewport_u: Vec3,
    viewport_v: Vec3 
}

impl OrthCamera {
    pub fn new() -> OrthCamera {

        OrthCamera {

            //TODO 
            viewport_u: Vec::new([0.0, 0.0, 0.0]),
            viewport_v: Vec::new([0.0, 0.0, 0.0])
        }
    }


    pub fn look_at() -> OrthCamera {
        todo!()
    }
}

impl Camera for OrthCamera {
    fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let aspect_ratio: f64 = WIDTH as f64 / HEIGHT as f64;

        let upper_left_corner = Vec::new([-1.0 * aspect_ratio, 1.0, 0.0]);

        let orig = upper_left_corner + 0.5  * self.viewport_u + 0.5 * self.viewport_v + x * self.viewport_u + y * self.viewport_v;

        Ray::new(orig, Vec::new([0.0, 0.0, 1.0]))
    }
}




#[cfg(test)]
mod tests {

    #[test]
    pub fn dummy_test() {
        assert_eq!(1 + 2, 3);
    }
}
