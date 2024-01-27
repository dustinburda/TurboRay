use crate::shape::Shape;
use crate::vec::Vec3;
use crate::matrix::Mat44;

pub struct BBox {
    min: Vec3,
    max: Vec3
}

impl BBox {
    pub fn union() -> BBox {
        todo!()
    }

    pub fn intersect() -> BBox {
        todo!()
    }

    pub fn transform(&mut self, mat: Mat44) -> BBox {
        todo!()
    }
}

// impl Shape for BBox {
//     todo!()
// }