use crate::shape::Shape;
use crate::matrix::Mat44;
use std::rc::Rc;

pub struct Instance {
    shape: Rc<dyn Shape>,
    obj_to_world: Mat44,
    world_to_obj: Mat44,
}