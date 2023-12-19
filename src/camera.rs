use crate::ray::Ray;

trait Camera {
    fn cast_ray(&self, x: f64, y: f64) -> Ray;
}

struct viewport {

}

struct ProjCamera {
    vw: viewport,
    width: f64,
    height: f64
}

// impl Camera for ProjCamera {

// }

struct OrthCamera {

}

// impl Camera for OrthCamera {

// }