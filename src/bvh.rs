use crate::shape::Shape;
use crate::bbox::BBox;
use std::rc::Rc;

type BVHNodeRef = Option<Box<BVHNode>>;

pub struct BVHNode {
    shape: Option<Rc<dyn Shape>>,
    aabb: BBox,
    left: BVHNodeRef,
    right: BVHNodeRef 
}


impl BVHNode {
    // pub fn new()
}