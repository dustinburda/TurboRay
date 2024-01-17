use std::{ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign}, rc::Rc};

// TODO: turn this into a union
#[derive(Copy, Debug)]
pub struct Color {
    data: [f64; 3]
}

impl Clone for Color {
    fn clone(&self) -> Color {
        *self
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b:f64) -> Color {
        Color {
            data: [r,g,b]
        }
    }

    pub fn r(&self) -> f64 {
        self.data[0]
    }

    pub fn g(&self) -> f64 {
        self.data[1]
    }

    pub fn b(&self) -> f64 {
        self.data[2]
    }

    pub fn hadamard_product(c1: Color, c2: Color) -> Color {
        Color::new(((c1.r()/255.0) * (c2.r()/255.0) * 255.0), 
                   ((c1.g()/255.0) * (c2.g()/255.0) * 255.0), 
                   ((c1.b()/255.0) * (c2.b()/255.0) * 255.0))
    }

    pub fn gamma(color: Color, g: f64) -> Color{
        if g > 1.0 || g < 0.0 {
            panic!("Gamma must be in [0.0, 1.0)");
        }

        let r_gamma: f64 = 255.0 * f64::powf(color.r() / 255.0, g);
        let g_gamma: f64 = 255.0 * f64::powf(color.g() / 255.0, g);
        let b_gamma: f64 = 255.0 * f64::powf(color.b() / 255.0, g);
        Color::new(r_gamma, g_gamma, b_gamma)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.r() + other.r(), self.g() + other.g(), self.b() + other.b())
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self * rhs.r(), self * rhs.g(), self * rhs.b())
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}


impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        Color::new(self.r() / rhs, self.g() / rhs, self.b() / rhs)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    const EPSILON: f64 = 0.00001;
    fn float_equal(x: f64, y: f64) -> bool {
        f64::abs(x - y) < EPSILON
    }

    #[test]
    fn color_accessor_test() {
        let c1: Color = Color::new(255.0, 234.0, 245.0);

        assert!(float_equal(c1.r(), 255.0));
        assert!(float_equal(c1.g(), 234.0));
        assert!(float_equal(c1.b(), 245.0));
    }

    #[test]
    fn color_hadamard_product_test() {
        let c1: Color = Color::new(234.0, 189.0, 245.0);
        let c2: Color = Color::new(84.0, 255.0, 239.0);

        let c3 = Color::hadamard_product(c1, c2);

        assert!(float_equal(c3.r(), 77.0823529));
        assert!(float_equal(c3.g(), 189.0));
        assert!(float_equal(c3.b(), 229.6274509));
    }

    #[test]
    fn color_scalar_mul_test() {
        let c1: Color = Color::new(234.0, 189.0, 245.0);

        let t = 0.58;

        let c2 = t * c1;
        let c3 = c1 * t;

        assert!(float_equal(c2.r(), 135.72));
        assert!(float_equal(c3.r(), 135.72));
        assert!(float_equal(c2.g(), 109.62));
        assert!(float_equal(c3.g(), 109.62));
        assert!(float_equal(c2.b(), 142.1));
        assert!(float_equal(c3.b(), 142.1));

    }


    #[test]
    fn color_scalar_mulassign_test() {
        let mut c1: Color = Color::new(234.0, 189.0, 245.0);

        let t = 0.58;

        c1 *= t;

        assert!(float_equal(c1.r(), 135.72));
        assert!(float_equal(c1.g(), 109.62));
        assert!(float_equal(c1.b(), 142.1));
    }



    #[test]
    fn color_scalar_div_test() {
        let c1: Color = Color::new(234.0, 189.0, 245.0);

        let t = 1.0/0.58;

        let c2 = c1 / t;

        assert!(float_equal(c2.r(), 135.72));
        assert!(float_equal(c2.g(), 109.62));
        assert!(float_equal(c2.b(), 142.1));
    }


    #[test]
    fn color_scalar_divassign_test() {
        let mut c1: Color = Color::new(234.0, 189.0, 245.0);

        let t = 1.0/0.58;

        c1 /= t;

        assert!(float_equal(c1.r(), 135.72));
        assert!(float_equal(c1.g(), 109.62));
        assert!(float_equal(c1.b(), 142.1));
    }




    #[test]
    fn color_add_test() {
        let c1: Color = Color::new(234.0, 189.0, 245.0);
        let c2: Color = Color::new(84.0, 255.0, 239.0);

        let c3 = c1 + c2;
        assert!(float_equal(c3.r(), 318.0));
        assert!(float_equal(c3.g(), 444.0));
        assert!(float_equal(c3.b(), 484.0));
    }


    #[test]
    fn color_addassign_test() {
        let mut c1: Color = Color::new(234.0, 189.0, 245.0);
        let c2: Color = Color::new(84.0, 255.0, 239.0);

        c1 += c2;
        assert!(float_equal(c1.r(), 318.0));
        assert!(float_equal(c1.g(), 444.0));
        assert!(float_equal(c1.b(), 484.0));
    }
}