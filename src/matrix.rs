#![feature(generic_const_exprs)]

use std::{ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign}, default};

pub type Mat44 = Matrix<4,4>;
pub type Mat33 = Matrix<3,3>;

// @param: R is the number of rows
// @param: C is the number of columns

struct Matrix<const R: usize, const C: usize> {
    mat_data: [[f64;C]; R]
}

impl<const R: usize, const C: usize> Default for Matrix<R,C> {
    fn default() -> Matrix<R,C> {
        Matrix {
            mat_data: [[0.0; C]; R]
        }
    }
}

/* 
- Implement Determinant
- Implement Transpose
- Implement Inverse
*/

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(mat_data: [[f64;C]; R]) -> Matrix<R,C> {
        Matrix {
            mat_data: mat_data
        }
    } 
}


impl<const R: usize, const C: usize> Index<usize> for Matrix<R,C> {
    type Output = [f64; C];

    fn index(&self, r: usize) -> &[f64; C] {
        return &self.mat_data[r];
    }
}


impl<const R: usize, const C: usize> IndexMut<usize> for Matrix<R,C> {
    fn index_mut(&mut self, r: usize) -> &mut [f64; C] {
        return &mut self.mat_data[r];
    }
}


impl<const R:usize, const C: usize> Add for Matrix<R,C> {
    type Output = Matrix<R,C>;

    fn add(self, other: Matrix<R,C>) -> Matrix<R,C> {
        let mut add_mat = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                add_mat[i][j] = self[i][j] + other[i][j]
            }
        }
        
        add_mat
    }
}


impl<const R: usize, const C: usize> AddAssign for Matrix<R,C> {
    fn add_assign(&mut self, other: Matrix<R, C>) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] += other[i][j]
            }
        }
    }
}


impl<const R: usize, const C: usize> Sub for Matrix<R,C> {
    type Output = Matrix<R,C>;

    fn sub(self, other: Self) -> Matrix<R,C> {
        let mut sub_mat = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                sub_mat[i][j] = self[i][j] - other[i][j];
            }
        }

        sub_mat
    }
}

impl<const R: usize, const C: usize> SubAssign for Matrix<R,C> {
    fn sub_assign(&mut self, other: Matrix<R,C>) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] -= other[i][j];
            }
        }
    }
}


impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn det(&self) -> f64 {
        todo!()
    }

    pub fn inv(&self) -> Matrix<R,C> {
        todo!()
    }

    pub fn t(&self) -> Matrix<R,C> {
        todo!()
    }
}

// Matrix-Matrix, Matrix-Vector, Matrix-Scalar[Commutative] multiplication

// Matrix-Scalar division

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    pub fn index_test() {
        let mut mat = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
                                                      [5.0, 6.0, 7.0, 8.0], 
                                                      [9.0, 10.0, 11.0, 12.0], 
                                                      [13.0, 14.0, 15.0, 16.0]]);
        
        assert_eq!(mat[3][3], 16.0);
        assert_eq!(mat[2][1], 10.0);
        assert_eq!(mat[0][2], 3.0);
    }

    #[test]
    pub fn index_mut_test() {
        let mut mat = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
                                                        [5.0, 6.0, 7.0, 8.0], 
                                                        [9.0, 10.0, 11.0, 12.0], 
                                                        [13.0, 14.0, 15.0, 16.0]]);

        mat[1][3] = 8.5;
        assert_eq!(mat[1][3], 8.5);

        mat[2][1] = -10.1;
        assert_eq!(mat[2][1], -10.1);
    }
}
