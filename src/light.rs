use crate::vec::{Vec3, Vec};

pub struct PointLight{
    diffuse_intensity: f64,
    specular_intensity: f64,
    position: Vec3
}

impl PointLight {
    pub fn new(diffuse_intensity: f64, specular_intensity: f64, position: Vec3) -> PointLight {
        if diffuse_intensity < 0.0 || diffuse_intensity > 1.0 {
            panic!("Intensity must be between 0.0 and 1.0!");
        }
        PointLight {
            diffuse_intensity: diffuse_intensity,
            specular_intensity: specular_intensity, 
            position: position
        }
    }

    pub fn diffuse_intensity(&self) -> f64 {
        self.diffuse_intensity
    }

    pub fn specular_intensity(&self) -> f64 {
        self.specular_intensity
    }

    pub fn pos(&self) -> Vec3 {
        self.position.clone()
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            diffuse_intensity: 1.0,
            specular_intensity: 1.0,
            position: Vec::new([0.0, 0.0, 0.0])
        }
    }
}