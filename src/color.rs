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


}