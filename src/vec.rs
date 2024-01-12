#![feature(generic_const_exprs)]

use std::{ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}, vec};

pub type Vec2 = Vec<2>;
pub type Vec3 = Vec<3>;
pub type Vec4 = Vec<4>;

// TODO: put this in its own module

#[derive(Copy, Clone)]
pub struct Vec<const N: usize> 
    where [f64; N]: Sized,
{
    data: [f64; N]
}

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

    // TODO: cross

    Vec::new(vec_data)
}

#[cfg(test)]
mod tests {
    
    #[test]
    pub fn equal() {
        assert_ne!(1,2);
    }
}