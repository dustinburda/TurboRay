use crate::vec::{Vec3, Vec};

pub struct PointLight{
    intensity: f64,
    position: Vec3
}

impl PointLight {
    pub fn new(intensity: f64, position: Vec3) -> PointLight {
        if intensity < 0.0 || intensity > 1.0 {
            panic!("Intensity must be between 0.0 and 1.0!");
        }
        PointLight {
            intensity: intensity, 
            position: position
        }
    }

    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    pub fn pos(&self) -> Vec3 {
        self.position.clone()
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            intensity: 1.0,
            position: Vec::new([0.0, 0.0, 0.0])
        }
    }
}