use crate::vec::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray { 
            orig: orig, 
            dir: dir
        }
    }

    pub fn set_orig(&mut self, orig: Vec3) -> () {
        self.orig = orig.clone();
    }

    pub fn set_dir(&mut self, dir: Vec3) -> () {
        self.dir = dir.clone();
    }

    // pub fn at(&self, t: f64) -> Vec3<f64> {
    //     self.orig + t * self.dir;
    // }
 }

