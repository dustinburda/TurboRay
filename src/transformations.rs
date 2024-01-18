use crate::matrix::{Matrix, Mat44};
use std::f64::consts::PI;

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
    Matrix::new([[1.0, 0.0,              0.0,                0.0],
                          [0.0, f64::cos(r), -f64::sin(r), 0.0],
                          [0.0, f64::sin(r), f64::cos(r),  0.0],
                          [0.0, 0.0,               0.0,               1.0]])
}


/* 
* @param: r, degrees in radians
*/
pub fn roty(r: f64) -> Mat44 {
    Matrix::new([[f64::cos(r),  0.0,  f64::sin(r), 0.0],
                           [0.0,               1.0,  0.0,              0.0],
                           [-f64::sin(r), 0.0, f64::cos(r), 0.0],
                           [0.0,               0.0,  0.0,              1.0]])
}


/* 
* @param: r, degrees in radians
*/
pub fn rotz(r: f64) -> Mat44 {
    Matrix::new([[f64::cos(r), -f64::sin(r), 0.0, 0.0],
                           [f64::sin(r), f64::cos(r), 0.0, 0.0],
                           [0.0,               0.0,              1.0, 0.0],
                           [0.0,               0.0,              0.0, 1.0]])
}

pub fn shear(x_z: f64, x_y: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Mat44 {
    Matrix::new([[1.0, x_y, x_z, 0.0],
                           [y_x, 1.0, y_z, 0.0],
                           [z_x, z_y, 1.0, 0.0],
                           [0.0, 0.0, 0.0, 1.0]])
}


#[cfg(test)]
mod tests {
    use crate::transformations::{translation, scale, rotx, roty, rotz, shear};
    use crate::vec::{Vec, Vec3};
    use std::f64::consts::PI;
    use crate::matrix::Matrix;

    const EPSILON: f64 = 0.0001;
    fn float_equal(x: f64, y: f64) -> bool {
        f64::abs(x - y) < EPSILON
    }
    #[test]
    fn translate_test() {
        let p: Vec3 = Vec::new([-3.0, 4.0, 5.0]);

        let translate1 = translation(5.0, -3.0, 2.0);

        // Translation matrix should translate point
        let p_translate = (translate1 * (p.homogenize())).dehomogenize();

        assert_eq!(p_translate, Vec::new([2.0, 1.0, 7.0]));

        // The inverse translation should transform the translated point to the original point
        let inv_translate1 = translate1.inv();

        let inv_p_translate = (inv_translate1 * p_translate.homogenize()).dehomogenize();

        assert_eq!(inv_p_translate, Vec::new([-3.0, 4.0, 5.0]));

        // Translations don't affect vectors
        let v: Vec3 = Vec::new([5.0, -3.0, 2.0]);

        let v_translate = (translate1 * v.homogenize_vec()).dehomogenize();

        assert_eq!(v_translate, Vec::new([5.0, -3.0, 2.0]));        
    }

    #[test]
    fn scale_test() {
        let p: Vec3 = Vec::new([-4.0, 6.0, 8.0]);

        let scale1 = scale(2.0, 3.0, 4.0);

        let p_scale = (scale1 * (p.homogenize())).dehomogenize();
        assert_eq!(p_scale, Vec::new([-8.0, 18.0, 32.0]));

        let inv_scale1 = scale1.inv();

        let inv_p_scale = (inv_scale1 * p.homogenize()).dehomogenize();
        assert_eq!(inv_p_scale, Vec::new([-2.0, 2.0, 2.0]));



        let v: Vec3 = Vec::new([-4.0, 6.0, 8.0]);

        let v_scale = (scale1 * v.homogenize_vec()).dehomogenize();
        assert_eq!(v_scale, Vec::new([-8.0, 18.0, 32.0]));
    }

    #[test]
    fn rotate_test() {
        let v1: Vec3 = Vec::new([0.0, 1.0, 0.0]);

        let rotPI_4x = rotx(PI / 4.0);
        let rotPI_2x = rotx(PI / 2.0);
        
        let v_x_pi_4 = (rotPI_4x * v1.homogenize()).dehomogenize();
        assert_eq!(v_x_pi_4, Vec::new([0.0, f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/2.0]));

        let v_x_pi_2 = (rotPI_2x * v1.homogenize()).dehomogenize();
        assert_eq!(v_x_pi_2, Vec::new([0.0, 0.0, 1.0]));

        let inv_rotPI_4x = rotx(PI / 4.0).inv();
        let inv_v_x_pi_4 =  (inv_rotPI_4x * v1.homogenize()).dehomogenize();
        assert_eq!(inv_v_x_pi_4, Vec::new([0.0, f64::sqrt(2.0)/2.0, -f64::sqrt(2.0)/2.0]));


        let v2: Vec3 = Vec::new([0.0, 0.0, 1.0]);

        let rotPI_4y = roty(PI / 4.0);
        let rotPI_2y = roty(PI / 2.0);

        let v_y_pi_4 = (rotPI_4y * v2.homogenize()).dehomogenize();
        assert_eq!(v_y_pi_4, Vec::new([f64::sqrt(2.0)/2.0, 0.0, f64::sqrt(2.0)/2.0]));

        let v_y_pi_2 = (rotPI_2y * v2.homogenize()).dehomogenize();
        assert_eq!(v_y_pi_2, Vec::new([1.0, 0.0, 0.0]));


        let rotPI_4z = rotz(PI / 4.0);
        let rotPI_2z = rotz(PI / 2.0);
        
        let v_z_pi_4 = (rotPI_4z * v1.homogenize()).dehomogenize();
        assert_eq!(v_z_pi_4, Vec::new([-f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/2.0, 0.0]));

        let v_z_pi_2 = (rotPI_2z * v1.homogenize()).dehomogenize();
        assert_eq!(v_z_pi_2, Vec::new([-1.0, 0.0, 0.0]));
    }


    #[test]
    fn shear_test() {
        let v1: Vec3 = Vec::new([2.0, 3.0, 4.0]);

        let shear1 = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let v1_shear1 = (shear1 * v1.homogenize()).dehomogenize();
        assert_eq!(v1_shear1, Vec::new([6.0, 3.0, 4.0]));

        let shear2 = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let v1_shear2 = (shear2 * v1.homogenize()).dehomogenize();
        assert_eq!(v1_shear2, Vec::new([5.0, 3.0, 4.0]));

        let shear3 = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let v1_shear3 = (shear3 * v1.homogenize()).dehomogenize();
        assert_eq!(v1_shear3, Vec::new([2.0, 5.0, 4.0]));

        let shear4 = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let v1_shear4 = (shear4 * v1.homogenize()).dehomogenize();
        assert_eq!(v1_shear4, Vec::new([2.0, 7.0, 4.0]));

        let shear5 = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let v1_shear5 = (shear5 * v1.homogenize()).dehomogenize();
        assert_eq!(v1_shear5, Vec::new([2.0, 3.0, 6.0]));

        let shear6 = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let v1_shear6 = (shear6 * v1.homogenize()).dehomogenize();
        assert_eq!(v1_shear6, Vec::new([2.0, 3.0, 7.0]));
    }

    #[test]
    fn integration_test() {
        let p: Vec3 = Vec::new([1.0, 0.0, 1.0]);

        let t1 = rotx(PI / 2.0);
        let t2 = scale(5.0, 5.0, 5.0);
        let t3 = translation(10.0, 5.0, 7.0);

        let t = t3 * t2 * t1;

        let p_prime = (t * p.homogenize()).dehomogenize();

        assert_eq!(p_prime, Vec::new([15.0, 0.0, 7.0]));
    }
}
