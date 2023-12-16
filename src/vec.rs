#![feature(generic_const_exprs)]

use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Copy, Clone)]
pub struct Vec<T, const N: usize> 
    where [T; N]: Sized,
{
    data: [T; N]
}

impl<T, const N: usize> Vec<T, N> {
    pub fn new(vec_data: [T; N]) -> Self {
        Vec { 
            data: vec_data
        }
    }
}

impl<T, const N: usize> Index<usize> for Vec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        return &self.data[index];
    } 
}

impl<T, const N: usize> IndexMut<usize> for Vec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        return &mut self.data[index];
    }
}

impl<T: std::ops::Add<Output = T> + Clone + Copy + Default, const N: usize> Add for Vec<T, N> {
    type Output = Vec<T, N>;

    fn add(self, other: Vec<T, N>) -> Vec<T, N> {
        let mut vec_data: [T; N] = [T::default(); N];

        for i in 0..N {
            vec_data[i] = self[i] + other[i];
        }

        Vec::new(vec_data)
    }
}

impl<T: Clone + Copy + Default + std::ops::Add<Output = T>, const N: usize> AddAssign for Vec<T, N> {
    fn add_assign(&mut self, other: Vec<T, N>) -> () {
        let mut vec_data: [T; N] = [T::default(); N];
        
        for i in 0..N {
            vec_data[i] = self[i] + other[i];
        }


        *self = Vec::new(vec_data);
    }
}

impl<T: Clone + Copy + Default + std::ops::Sub<Output = T>, const N: usize> Sub for Vec<T, N> {
    type Output = Vec<T, N>;
    
    fn sub(self, other: Vec<T, N>) -> Vec<T, N> {
        let mut vec_data: [T; N] = [T::default(); N];

        for i in 0..N {
            vec_data[i] = self[i] - other[i];
        }

        Vec::new(vec_data)
    }
} 

impl<T: Copy + Clone + Default + std::ops::Sub<Output = T>, const N: usize> SubAssign for Vec<T, N> {
    fn sub_assign(&mut self, other: Vec<T, N>) {
        let mut vec_data: [T; N] = [T::default(); N];

        for i in 0..N {
            vec_data[i] = self[i] - other[i]
        }

        Vec::new(vec_data);
    }
}

// impl<T: Clone + Copy + Default + std::ops::Mul<Output = T, const N: usize> Mul<T> for Vec<T,N> {

// }


impl<T:Copy + Clone + Default, const N: usize> Vec<T,N> {
    pub fn magnitude(&self) -> T {
        T::default()
    }

    pub fn normal(&self) -> Vec<T, N> {
        let mut vec_data: [T;N] = [T::default(); N];

        Vec::new(vec_data)
    }

    pub fn dot(&self, other: Vec<T,N>) -> T {
        T::default()
    }

    pub fn homogenize(&self) -> Vec<T, {N+1}> {
        let mut vec_data: [T; {N+1}] = [T::default(); {N+1}];

        Vec::new(vec_data)
    }
}

