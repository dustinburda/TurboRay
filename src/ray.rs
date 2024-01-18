use crate::vec::Vec3;
use crate::matrix::Matrix;

#[derive(Debug)]
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
        self.orig = orig;
    }

    pub fn orig(&self) -> Vec3 {
        self.orig
    }

    pub fn set_dir(&mut self, dir: Vec3) -> () {
        self.dir = dir;
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }

    pub fn transform(&self, mat: &Matrix<4,4>) -> Ray {
        Ray {
            orig: (*mat * self.orig.homogenize()).dehomogenize(),
            dir: (*mat * self.dir.homogenize_vec()).dehomogenize()
        }
    }
 }

#[cfg(test)]
mod tests {
    use crate::vec::Vec;
    use crate::ray::Ray;

    #[test]
    pub fn set_orig_test() {
        let mut r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 0.0]));

        r.set_orig(Vec::new([1.0, 2.0, 3.0]));

        assert_eq!(Vec::new([1.0, 2.0, 3.0]), r.orig);
    }

    #[test]
    pub fn set_dir_test() {
        let mut r = Ray::new(Vec::new([0.0, 0.0, 0.0]), Vec::new([0.0, 0.0, 0.0]));

        r.set_dir(Vec::new([1.0, 2.0, 3.0]));

        assert_eq!(Vec::new([1.0, 2.0, 3.0]), r.dir);
    }

    #[test]
    pub fn at_test() {
        let mut r = Ray::new(Vec::new([1.0, 2.0, 3.0]), Vec::new([1.0, 2.0, 1.0]));

        assert_eq!(r.at(2.0), Vec::new([3.0, 6.0, 5.0]));
        assert_eq!(r.at(-2.0), Vec::new([-1.0, -2.0, 1.0]));
        assert_eq!(r.at(1.5), Vec::new([2.5, 5.0, 4.5]));
    }

    pub fn transform_test() {
        
    }

}

