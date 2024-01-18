#![feature(generic_const_exprs)]

use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use num_traits::{abs, float};


pub type Vec2 = Vec<2>;
pub type Vec3 = Vec<3>;
pub type Vec4 = Vec<4>;

// TODO: put this in a math module
// TODO: homogenize normal

const EPSILON: f64 = 0.00001;

fn float_equal(x: f64, y: f64) -> bool {
    return abs(x - y) < EPSILON;
}


#[derive(Copy, Clone, Debug)]
pub struct Vec<const N: usize> 
    where [f64; N]: Sized,
{
    data: [f64; N]
}


impl<const N: usize> PartialEq for Vec<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if !float_equal(self[i], other[i]) {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> Eq for Vec<N> {}

impl<const N: usize> Vec<N> {
    pub fn new(vec_data: [f64; N]) -> Self {
        Vec { 
            data: vec_data
        }
    }
}

impl<const N: usize> Index<usize> for Vec<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        return &self.data[index];
    } 
}

impl<const N: usize> IndexMut<usize> for Vec<N> {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        return &mut self.data[index];
    }
}

impl<const N: usize> Add for Vec<N> {
    type Output = Vec<N>;

    fn add(self, other: Vec<N>) -> Vec<N> {
        let mut vec_data: [f64; N] = [0.0 as f64; N];

        for i in 0..N {
            vec_data[i] = self[i] + other[i];
        }

        Vec::new(vec_data)
    }
}

impl<const N: usize> AddAssign for Vec<N> {
    fn add_assign(&mut self, other: Vec<N>) { 
        for i in 0..N {
            self[i] += other[i];
        }
    }
}

impl<const N: usize> Sub for Vec<N> {
    type Output = Vec<N>;
    
    fn sub(self, other: Vec<N>) -> Vec<N> {
        let mut vec_data: [f64; N] = [0.0 as f64; N];

        for i in 0..N {
            vec_data[i] = self[i] - other[i];
        }

        Vec::new(vec_data)
    }
} 

impl<const N: usize> SubAssign for Vec<N> {
    fn sub_assign(&mut self, other: Vec<N>) {
        for i in 0..N {
            self[i] -= other[i]
        }
    }
}

impl<const N: usize> Mul<f64> for Vec<N> {
    type Output = Vec<N>;

    fn mul(self, other: f64) -> Vec<N> {
        let mut vec_data: [f64; N] = self.data.clone();

        for i in 0..N {
            vec_data[i] *= other;
        }

        Vec::new(vec_data)
    }
}

impl<const N: usize> Mul<Vec<N>> for f64 {
    type Output = Vec<N>;

    fn mul(self, other: Vec<N>) -> Vec<N> {
        let mut vec_data: [f64; N] = other.data.clone();

        for i in 0..N {
            vec_data[i] *= self;
        }

        Vec::new(vec_data)
    }
}

impl<const N: usize> MulAssign<f64> for Vec<N> {
    fn mul_assign(&mut self, other: f64) {
        for i in 0..N {
            self[i] *= other;
        }
    }
}

impl<const N: usize> Div<f64> for Vec<N> {
    type Output = Vec<N>;

    fn div(self, other: f64) -> Vec<N> {
        let mut vec_data: [f64; N] = [0.0 as f64; N];

        for i in 0..N {
            vec_data[i] = self[i] / other;
        }

        Vec::new(vec_data)
    }
}

impl<const N: usize> DivAssign<f64> for Vec<N> {
    fn div_assign(&mut self, other: f64) {
        for i in 0..N {
            self[i] /= other;
        }
    }
}

impl<const N: usize> Vec<N> {
    pub fn magnitude(&self) -> f64 {
        let mut mag: f64 = 0.0 as f64;

        for i in 0..N {
            mag += self[i] * self[i];
        }

        mag = f64::sqrt(mag);

        mag
    }

    pub fn normal(&self) -> Vec<N> {
        let mut vec_data: [f64;N] = self.data;

        let mag: f64 = self.magnitude();
        for i in 0..N {
            vec_data[i] /= mag;
        }

        Vec::new(vec_data)
    }

    pub fn homogenize(&self) -> Vec<{N+1}> {
        let mut vec_data: [f64; {N+1}] = [0.0 as f64; {N+1}];

        for i in 0..N {
            vec_data[i] = self[i];
        }
        vec_data[N] = 1.0 as f64;

        Vec::new(vec_data)
    }

    pub fn homogenize_vec(&self) -> Vec<{N+1}> {
        let mut vec_data: [f64; {N+1}] = [0.0 as f64; {N+1}];

        for i in 0..N {
            vec_data[i] = self[i];
        }
        vec_data[N] = 0.0 as f64;

        Vec::new(vec_data)
    }

    pub fn dehomogenize(&self) -> Vec<{N-1}> {
        if {N-1} <= 0 {
            panic!("Can't dehomogenize a vector of 1-dimension or lower!");
        }

        let mut vec_data: [f64; {N-1}] = [0.0; {N-1}];

        for i in 0..N-1 {
            vec_data[i] = if self[{N-1}] != 0.0 {self[i] / self[{N-1}]} else { self[i] };
        }

        Vec::new(vec_data)
    }
}


pub fn dot<const N: usize>(v1: Vec<N>, v2: Vec<N>) -> f64 {
    let mut dot_product = 0.0 as f64;

    for i in 0..N {
        dot_product += v1[i] * v2[i];
    }

    dot_product
}

pub fn cross(v1: Vec<3>, v2: Vec<3>) -> Vec<3> {
    let mut vec_data: [f64; 3] = [0.0 as f64; 3];

    vec_data[0] = v1[1]*v2[2] - v2[1] * v1[2];
    vec_data[1] = - (v1[0]*v2[2] - v2[0] * v1[2]);
    vec_data[2] = v1[0]*v2[1] - v2[0] * v1[1];

    Vec::new(vec_data)
}

#[cfg(test)]
mod tests {

    use crate::vec::Vec;
    use crate::vec::{dot, cross, float_equal};
    
    #[test]
    pub fn index_test() {
        let v: Vec<4> = Vec::new([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v[3], 4.0);
    }

    #[test]
    pub fn index_mut_test() {
        let mut v: Vec<4> = Vec::new([1.0, 2.0, 3.0, 4.0]);
        v[0] += 1.0;
        assert_eq!(v[0], 2.0);

        v[1] -= 3.45;
        assert!(float_equal(v[1], -1.45));

        v[2] *= 2.1;
        assert!(float_equal(v[2], 6.3));
        
        v[3] /= 0.4;
        assert!(float_equal(v[3], 10.0));
    }


    #[test]
    pub fn add_test() {
        let v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);
        let v2: Vec<3> = Vec::new([2.1, -1.2, -3.3]);

        let v3: Vec<3> = v1 + v2;

        assert_eq!(v3, Vec::new([3.1, 0.8, -0.3]));
    }

    #[test]
    pub fn add_assign_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);
        let v2: Vec<3> = Vec::new([2.1, -1.2, -3.3]);

        v1 += v2;

        assert_eq!(v1, Vec::new([3.1, 0.8, -0.3]));
    }

    #[test]
    pub fn sub_test() {
        let v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);
        let v2: Vec<3> = Vec::new([2.1, -1.2, -3.3]);

        let v3: Vec<3> = v1 - v2;

        assert_eq!(v3, Vec::new([-1.1, 3.2, 6.3]));
    }

    #[test]
    pub fn sub_assign_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);
        let v2: Vec<3> = Vec::new([2.1, -1.2, -3.3]);

        v1 -= v2;

        assert_eq!(v1, Vec::new([-1.1, 3.2, 6.3]));
    }


    #[test]
    pub fn mul_test() {
        let v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let t: f64 = 2.1;

        let v3 = v1 * t;
  
        assert_eq!(v3, Vec::new([2.1, 4.2, 6.3]));

        let v4 = t * v1;

        assert_eq!(v4, Vec::new([2.1, 4.2, 6.3]));
    }

    #[test]
    pub fn mul_assign_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let t: f64 = 2.1;

        v1 *= t;

        assert_eq!(v1, Vec::new([2.1, 4.2, 6.3]));
    }

    #[test]
    pub fn div_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let t: f64 = 2.1;

        let v2 =  v1 / t;

        assert_eq!(v2, Vec::new([0.476190476, 0.952380952, 1.428571429]));
    }


    #[test]
    pub fn div_assign_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let t: f64 = 2.1;

        v1 /= t;

        assert_eq!(v1, Vec::new([0.476190476, 0.952380952, 1.428571429]));
    }

    #[test]
    pub fn magnitude_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        assert!(float_equal(v1.magnitude(), 3.741657387));


        let mut v2: Vec<3> = Vec::new([1.0, -2.0, 3.0]);

        assert!(float_equal(v1.magnitude(), 3.741657387));
    }

    #[test]
    pub fn normal_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let v2 = v1.normal();

        assert!(float_equal(v2[0], v1[0] / 3.741657387));
        assert!(float_equal(v2[1], v1[1] / 3.741657387));
        assert!(float_equal(v2[2], v1[2] / 3.741657387));
    }

    #[test]
    pub fn homogenize_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let v2 = v1.homogenize();

        assert!(float_equal(v2[0], v1[0]));
        assert!(float_equal(v2[1], v1[1]));
        assert!(float_equal(v2[2], v1[2]));
        assert!(float_equal(v2[3], 1.0));
    }

    #[test]
    pub fn dehomogenize_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);

        let v2 = v1.dehomogenize();

        assert!(float_equal(v2[0], 1.0/3.0));
        assert!(float_equal(v2[1], 2.0/3.0));
    }

    #[test]
    #[should_panic]
    pub fn dehomogenize_fail_test() {
        let mut v1: Vec<1> = Vec::new([1.0]);

        let v2 = v1.dehomogenize();
    }

    #[test]
    pub fn dot_test() {
        let mut v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);
        let mut v2: Vec<3> = Vec::new([1.0, -2.0, 1.0]);

        let dot_product12 = dot(v1, v2);

        assert!(float_equal(dot_product12, 0.0));


        let mut v3: Vec<3> = Vec::new([0.67, 2.34, -3.72]);
        let mut v4: Vec<3> = Vec::new([1.32, 1.19, 1.59]);

        let dot_product34 = dot(v3, v4);

        assert!(float_equal(dot_product34, -2.2458));
    }

    #[test]
    pub fn cross_test() {
        let v1: Vec<3> = Vec::new([1.0, 2.0, 3.0]);
        let v2: Vec<3> = Vec::new([2.0, 3.0, 1.0]);

        let v3 = cross(v1, v2);

        assert_eq!(v3, Vec::new([-7.0, 5.0, -1.0]));
    }

}