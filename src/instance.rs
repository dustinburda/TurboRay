use crate::shape::Shape;
use crate::matrix::Mat44;

pub struct Instance {
    shape: Box<dyn Shape>,
    obj_to_world: Mat44,
    world_to_obj: Mat44,
}