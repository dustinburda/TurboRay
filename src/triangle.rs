use crate::color::Color;
use crate::vec::Vec3;
use crate::shape::Shape;

struct Vertex {
    pub pos: Vec3,
    pub color: Option<Color>,
    pub normal: Option<Vec3>
}

// Accounts for winding order
pub struct Triangle {
    verts: [Vertex; 3]
}