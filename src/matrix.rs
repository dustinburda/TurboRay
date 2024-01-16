#![feature(generic_const_exprs)]

use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub type Mat44 = Matrix<4,4>;
pub type Mat33 = Matrix<3,3>;

// @param: R is the number of rows
// @param: C is the number of columns
struct Matrix<const R: usize, const C: usize> {
    data: [[f64;C]; R]
}

/* 
- Implement OPS
- Implement Determinant
- Implement Transpose
- Implement Inverse
*/

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new<R,C>() 
}