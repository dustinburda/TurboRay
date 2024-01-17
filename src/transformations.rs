use crate::matrix::{Matrix, Mat44};

pub fn translation(Tx: f64, Ty: f64, Tz: f64) -> Mat44 {
    Matrix::new([[1.0, 0.0, 0.0, Tx],
                           [0.0, 1.0, 0.0, Ty],
                           [0.0, 0.0, 1.0, Tz],
                           [0.0, 0.0, 0.0, 1.0]])
}

pub fn scale(x: f64, y: f64, z:f64) -> Mat44 {
    Matrix::new([[x, 0.0, 0.0, 0.0],
                           [0.0, y, 0.0, 0.0],
                           [0.0, 0.0, z, 0.0],
                           [0.0, 0.0, 0.0, 1.0]])
}

/* 
* @param: r, degrees in radians
*/
pub fn rotx(r: f64) -> Mat44 {
    todo!()
}


/* 
* @param: r, degrees in radians
*/
pub fn roty(r: f64) -> Mat44 {
    todo!()
}


/* 
* @param: r, degrees in radians
*/
pub fn rotz(r: f64) -> Mat44 {
    todo!()
}

pub fn shear(x_z: f64, x_y: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Mat44 {
    Matrix::new([[1.0, x_y, x_z, 0.0],
                           [y_x, 1.0, y_z, 0.0],
                           [z_x, z_y, 1.0, 0.0],
                           [0.0, 0.0, 0.0, 1.0]])
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn test1() {
        assert_eq!(1,1);
    }
}
