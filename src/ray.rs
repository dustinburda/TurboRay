use crate::vec::Vec;

struct Ray {
    orig: Vec<f64, 3>,
    dir: Vec<f64, 3>
}

impl Ray {
    pub fn new(orig: Vec<f64, 3>, dir: Vec<f64, 3>) -> Ray {
        Ray { 
            orig: orig, 
            dir: dir
        }
    }

    pub fn set_orig(&mut self, orig: Vec<f64, 3>) -> () {
        self.orig = orig.clone();
    }

    pub fn set_dir(&mut self, dir: Vec<f64, 3>) -> () {
        self.dir = dir.clone();
    }

    // pub fn at(&self, t: f64) -> Vec3<f64> {
    //     self.orig + t * self.dir;
    // }
 }

