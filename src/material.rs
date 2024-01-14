use crate::color::Color;

pub enum Material {
    Matte(Color),
    Plasic,
    Metal,
    Mirror,
    Glass
}