#![feature(generic_const_exprs)]

use std::{ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg}, default};

use crate::vec::Vec;

pub type Mat44 = Matrix<4,4>;
pub type Mat33 = Matrix<3,3>;

// @param: R is the number of rows
// @param: C is the number of columns
#[derive(Copy, Clone)]
pub struct Matrix<const R: usize, const C: usize> {
    mat_data: [[f64;C]; R]
}

impl<const R: usize, const C: usize> Default for Matrix<R,C> {
    fn default() -> Matrix<R,C> {
        Matrix {
            mat_data: [[0.0; C]; R]
        }
    }
}

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

impl<const R: usize, const C: usize> Neg for Matrix<R,C> {
    type Output = Matrix<R,C>;

    fn neg(self) -> Matrix<R,C> {
        let mut neg_mat: Matrix<R,C> = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                neg_mat[i][j] = self[i][j] * (-1.0);
            }
        }

        neg_mat
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

impl<const R: usize, const C: usize, const R2: usize> Mul<Matrix<C,R2>> for Matrix<R,C> {
    type Output = Matrix<R, R2>;

    fn mul(self, other: Matrix<C, R2>) -> Matrix<R,R2> {
        let mut mul_mat = Matrix::default();

        for i in 0..R {
            for j in 0..R2 {
                let mut entry: f64 = 0.0;
                for k in 0..C {
                    entry += self[i][k] * other[k][j];
                }
                mul_mat[i][j] = entry;
            }
        }

        mul_mat
    }
}

impl<const R: usize, const C: usize> Mul<Vec<C>> for Matrix<R,C> {
    type Output = Vec<R>;

    fn mul(self, other: Vec<C>) -> Vec<R> {
        let mut mul_vec = Vec::new([0.0; R]);

        for i in 0..R {
            let mut entry: f64 = 0.0;
            for j in 0..C {
                entry += self[i][j] * other[j];
            }
            mul_vec[i] = entry;
        }

        mul_vec
    }
}

impl<const R: usize, const C: usize> Mul<f64> for Matrix<R,C> {
    type Output = Matrix<R,C>;

    fn mul(self, other: f64) -> Matrix<R,C> {
        let mut mat_mul = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                mat_mul[i][j] = self[i][j] * other; 
            }
        }

        mat_mul
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<R,C>> for f64 {
    type Output = Matrix<R,C>;

    fn mul(self, other: Matrix<R,C>) -> Matrix<R,C> {
        let mut mat_mul = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                mat_mul[i][j] = self * other[i][j]; 
            }
        }

        mat_mul
    }
}

impl<const R: usize, const C: usize> MulAssign<f64> for Matrix<R,C> {
    fn mul_assign(&mut self, other: f64) {

        for i in 0..R {
            for j in 0..C {
                self[i][j] *= other;
            }
        }
    }
}

impl<const R: usize, const C: usize> Div<f64> for Matrix<R,C> {
    type Output = Matrix<R,C>;
    
    fn div(self, other: f64) -> Matrix<R,C> {
        let mut div_mat = Matrix::default();

        for i in 0..R {
            for j in 0..C {
                div_mat[i][j] = self[i][j] / other;
            }
        }

        div_mat
    }
}

impl<const R: usize, const C: usize> DivAssign<f64> for Matrix<R,C> {
    fn div_assign(&mut self, other: f64) {
        for i in 0..R {
            for j in 0..C {
                self[i][j] /= other;
            }
        }
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn det(&self) -> f64 {
        if(R != C) {
            panic!("Can't take determinant of non-square matrix!")
        }

        let mut mat_copy = (*self).clone();
        let mut determinant: f64 = 1.0;

        // pick a row
        'outer: for i in 0..C {
            // for each row below it
            for j in i+1..R {
                if(mat_copy[i][i] == 0.0) {
                    break 'outer;
                }
                let ratio = mat_copy[j][i] / mat_copy[i][i];

                for k in i..R {
                    mat_copy[j][k] -= ratio * mat_copy[i][k];
                }
                // subtract out 
            }
        }

        for i in 0..R {
            determinant *= mat_copy[i][i];
        }


        determinant
    }

    pub fn identity() -> Matrix<R,C> {
        if(R != C) {
            panic!("There does not a exist a non-square identity matrix!"); 
        }

        let mut mat_data = [[0.0; C]; R];

        for i in 0..R {
            mat_data[i][i] = 1.0;
        }

        Matrix::new(mat_data)    
    }

    pub fn inv(&self) -> Matrix<R,C> {
        if(R != C) {
            panic!("Can not take inverse of non-square matrix!");
        }

        let mut m = (*self).clone();
        let mut inv: Matrix<R,C> = Matrix::identity();

    for column in 0..R {
        
        // Step 1, pick a pivot
        if m[column][column] == 0.0 {
            let mut alt_row = column;

            for row in 0..R {
                if f64::abs(m[alt_row][column]) > f64::abs(m[row][column]) {
                    alt_row = row;
                }
            }

            if alt_row == column {
                panic!("Matrix not invertible!");
            }
            else {
                // std::mem::swap(&mut m[alt_row], &mut m[column]);
                
                let temp1 = m[alt_row].clone();
                m[alt_row] = m[column].clone();
                m[column] = temp1;


                // std::mem::swap(&mut inv[alt_row], &mut inv[column]);

                let temp2 = m[alt_row].clone();
                m[alt_row] = m[column].clone();
                m[column] = temp2;
            }
                
        }

        // Step 2, eliminate all elements below diagonal

        for row in (column + 1)..R {
            let mut coeff: f64 = m[row][column] / m[column][column];

            for col in 0..R {
                m[row][col] -= m[column][col] * coeff;
                inv[row][col] -= inv[column][col] * coeff;
            }

            // m[row][column] = 0.0;
        }
    }

    // Step 3, scale diagonal elements to be equal to 1.0
    for i in 0..R {
        let mut coeff: f64 = m[i][i];
        for j in 0..R {
            m[i][j] /= coeff;
            inv[i][j] /= coeff;
        }
       // m[i][i] = 1.0;
    }

    // Step 4, eliminate all elements above diagonal
    for col in 0..4 {
        for row in (0..col).rev() {
            let mut coeff = m[row][col];

            for j in 0..R {
                m[row][j] -= coeff * m[col][j];
                inv[row][j] -= coeff * inv[col][j];
            }

           //  m[col][row] = 0.0;
        }
    }

        inv
    }

    pub fn t(&self) -> Matrix<C,R> {
        let mut t_mat = Matrix::default();  

        for i in 0..R {
            for j in 0..C {
                t_mat[j][i] = self[i][j];
            }
        }

        t_mat
    }
}



#[cfg(test)]
mod tests {
    use num_traits::float;
    use crate::vec::Vec;

    use crate::matrix::Matrix;
    
    const EPSILON: f64 = 0.00001;
    pub fn float_equal(x: f64, y:f64) -> bool {
        f64::abs(x -y) < EPSILON
    }

    #[test]
    fn index_test() {
        let mut mat = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
                                                      [5.0, 6.0, 7.0, 8.0], 
                                                      [9.0, 10.0, 11.0, 12.0], 
                                                      [13.0, 14.0, 15.0, 16.0]]);
        
        assert_eq!(mat[3][3], 16.0);
        assert_eq!(mat[2][1], 10.0);
        assert_eq!(mat[0][2], 3.0);
    }

    #[test]
    fn index_mut_test() {
        let mut mat = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
                                                        [5.0, 6.0, 7.0, 8.0], 
                                                        [9.0, 10.0, 11.0, 12.0], 
                                                        [13.0, 14.0, 15.0, 16.0]]);

        mat[1][3] = 8.5;
        assert_eq!(mat[1][3], 8.5);

        mat[2][1] = -10.1;
        assert_eq!(mat[2][1], -10.1);
    }

    #[test]
    fn add_test() {

        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let mut mat2 = Matrix::new([[-1.0, -2.0, 6.0, 3.1], 
            [2.0, -7.0, -7.0, 7.73], 
            [2.0, -10.0, -11.0, 12.5], 
            [13.0, -14.2, 15.0, -16.1]]);

        let mat3 = mat1 + mat2;


        assert!(float_equal(mat3[0][0], 0.0));
        assert!(float_equal(mat3[2][3], 24.5));
        assert!(float_equal(mat3[1][3], 15.73));  
        assert!(float_equal(mat3[3][3], -0.1));  
    }

    #[test]
    fn add_assign_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let mut mat2 = Matrix::new([[-1.0, -2.0, 6.0, 3.1], 
            [2.0, -7.0, -7.0, 7.73], 
            [2.0, -10.0, -11.0, 12.5], 
            [13.0, -14.2, 15.0, -16.1]]);

        mat1 += mat2;


        assert!(float_equal(mat1[0][0], 0.0));
        assert!(float_equal(mat1[2][3], 24.5));
        assert!(float_equal(mat1[1][3], 15.73));  
        assert!(float_equal(mat1[3][3], -0.1));  
    }

    #[test]
    fn sub() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let mut mat2 = Matrix::new([[-1.0, -2.0, 6.0, 3.1], 
            [2.0, -7.0, -7.0, 7.73], 
            [2.0, -10.0, -11.0, 12.5], 
            [13.0, -14.2, 15.0, -16.1]]);

        let mat3 = mat1 - mat2;


        assert!(float_equal(mat3[0][0], 2.0));
        assert!(float_equal(mat3[2][3], -0.5));
        assert!(float_equal(mat3[1][3], 0.27));  
        assert!(float_equal(mat3[3][3], 32.1)); 
    }

    #[test]
    fn sub_assign_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let mut mat2 = Matrix::new([[-1.0, -2.0, 6.0, 3.1], 
            [2.0, -7.0, -7.0, 7.73], 
            [2.0, -10.0, -11.0, 12.5], 
            [13.0, -14.2, 15.0, -16.1]]);

        mat1 -= mat2;


        assert!(float_equal(mat1[0][0], 2.0));
        assert!(float_equal(mat1[2][3], -0.5));
        assert!(float_equal(mat1[1][3], 0.27));  
        assert!(float_equal(mat1[3][3], 32.1)); 
    }

    #[test]
    fn mat_mat_mul_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let mut mat2 = Matrix::new([[-1.0, -2.0, 6.0, 3.1], 
            [2.0, -7.0, -7.0, 7.73], 
            [2.0, -10.0, -11.0, 12.5], 
            [13.0, -14.2, 15.0, -16.1]]);
        
        let mat3 = mat1 * mat2;

        assert!(float_equal(mat3[0][0], 61.0));
        assert!(float_equal(mat3[1][2], 31.0));
        assert!(float_equal(mat3[3][3], 78.42));

    }

    #[test]
    fn mat_vec_mul_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);
        
        let vec1 = Vec::new([1.0, 2.0, 3.0, 4.0]);

        let vec2 = mat1 * vec1;
        
        assert!(float_equal(vec2[0], 30.0));
        assert!(float_equal(vec2[1], 70.0));
        assert!(float_equal(vec2[2], 110.0));
        assert!(float_equal(vec2[3], 150.0));
    }   
    
    #[test]
    fn mat_scalar_mul_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let t: f64 = 2.0;

        let mat2 = mat1 * t;
        let mat3 = t * mat1;


        assert!(float_equal(mat2[0][0], 2.0));
        assert!(float_equal(mat2[2][3], 24.0));
        assert!(float_equal(mat2[1][2], 14.0));  
        assert!(float_equal(mat2[3][1], 28.0)); 


        assert!(float_equal(mat3[0][0], 2.0));
        assert!(float_equal(mat3[2][3], 24.0));
        assert!(float_equal(mat3[1][2], 14.0));  
        assert!(float_equal(mat3[3][1], 28.0)); 
    }


    #[test]
    fn mat_scalar_mulassign_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let t: f64 = -1.5;

        mat1 *= t;


        assert!(float_equal(mat1[0][0], -1.5));
        assert!(float_equal(mat1[2][3], -18.0));
        assert!(float_equal(mat1[1][2], -10.5));  
        assert!(float_equal(mat1[3][1], -21.0)); 
    }

    #[test]
    fn mat_scalar_div_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let t: f64 = -2.0;

        let mat2 = mat1 / t;
        
        assert!(float_equal(mat2[0][0], -0.5));
        assert!(float_equal(mat2[2][3], -6.0));
        assert!(float_equal(mat2[1][2], -3.5));  
        assert!(float_equal(mat2[3][1], -7.0)); 
    }

    #[test]
    fn mat_scalar_divassign_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let t: f64 = -2.0;

        mat1 /= t;
        
        assert!(float_equal(mat1[0][0], -0.5));
        assert!(float_equal(mat1[2][3], -6.0));
        assert!(float_equal(mat1[1][2], -3.5));  
        assert!(float_equal(mat1[3][1], -7.0)); 
    }

    #[test]
    fn transpose_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0]]);
        
        let mat2 = mat1.t();

        assert!(float_equal(mat2[3][2], mat1[2][3]));
        assert!(float_equal(mat2[2][1], mat1[1][2]));
        assert!(float_equal(mat2[0][2], mat1[2][0]));
    }

    #[test]
    #[should_panic]
    fn det_nonsquare_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0]]);

        let determinant = mat1.det();
    }

    #[test]
    fn det_test() {
        let mut mat1 = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);
        
        let determinant1: f64 = mat1.det();
        assert!(float_equal(determinant1, 0.0));


        let mut mat2 = Matrix::new([[1.0, 0.0, 0.0, 0.0], 
            [0.0, 1.0, 0.0, 0.0], 
            [0.0, 0.0, 1.0, 0.0], 
            [0.0, 0.0, 0.0, 1.0]]);

        let determinant2: f64 = mat2.det();
        assert_eq!(determinant2, 1.0);


        let mut mat3 = Matrix::new([[-2.0, -8.0, 3.0, 5.0], 
            [-3.0, 1.0, 7.0, 3.0], 
            [1.0, 2.0, -9.0, 6.0], 
            [-6.0, 7.0, 7.0, -9.0]]);
        
        let determinant3: f64 = mat3.det();
        assert!(float_equal(determinant3, -4071.0));


        let mut mat4 = Matrix::new([[-4.0, 2.0, -2.0, -3.0], 
            [9.0, 6.0, 2.0, 6.0], 
            [0.0, -5.0, 1.0, 5.0], 
            [0.0, 0.0, 0.0, 0.0]]);
        
        let determinant4: f64 = mat4.det();
        assert!(float_equal(determinant4, 0.0));


        let mut mat5 = Matrix::new([[6.0, 4.0, 4.0, 4.0], 
            [5.0, 5.0, 7.0, 6.0], 
            [4.0, -9.0, 3.0, -7.0], 
            [9.0, 1.0, 7.0, -6.0]]);
        
        let determinant5: f64 = mat5.det();
        assert!(float_equal(determinant5, -2120.0));
    }

    #[test]
    fn inv_test() {
        let mat1: Matrix<4,4> = Matrix::identity();

        let mat_inv1 = mat1.inv();

        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert!(float_equal(mat_inv1[i][j], 1.0))
                } else {
                    assert!(float_equal(mat_inv1[i][j], 0.0))
                }
            }
        }


       let mat2 = Matrix::new([[-5.0, 2.0, 6.0, -8.0], 
                                                       [1.0, -5.0, 1.0, 8.0], 
                                                       [7.0, 7.0, -6.0, -7.0], 
                                                       [1.0, -3.0, 7.0, 4.0]]);
        
        let mat2_inv = mat2.inv();

        let identity2 = mat2 * mat2_inv;
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert!(float_equal(identity2[i][j], 1.0))
                } else {
                    assert!(float_equal(identity2[i][j], 0.0))
                }
            }
        }


        let mat3 = Matrix::new([[8.0, -5.0, 9.0, 2.0], 
                                                        [7.0, 5.0, 6.0, 1.0], 
                                                        [-6.0, 0.0, 9.0, 6.0], 
                                                        [-3.0, 0.0, -9.0, -4.0]]);

        let mat3_inv = mat3.inv();

        let identity3 = mat3 * mat3_inv;
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert!(float_equal(identity3[i][j], 1.0))
                } else {
                    assert!(float_equal(identity3[i][j], 0.0))
                }
            }
        }
    }

    #[test]
    fn neg_mat_test() {
        let mut mat = Matrix::new([[1.0, 2.0, 3.0, 4.0], 
            [5.0, 6.0, 7.0, 8.0], 
            [9.0, 10.0, 11.0, 12.0], 
            [13.0, 14.0, 15.0, 16.0]]);

        let mat = -mat;

        assert_eq!(mat[3][3], -16.0);
        assert_eq!(mat[2][1], -10.0);
        assert_eq!(mat[0][2], -3.0);
    }



}
