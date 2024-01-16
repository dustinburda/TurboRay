use crate::vec::Vec3;

struct PointLight{
    intensity: f64,
    position: Vec3
}

impl PointLight {
    pub fn new(intensity: f64, position: Vec3) -> PointLight {
        PointLight {
            intensity: intensity, 
            position: position
        }
    }
}