use crate::shape::Shape;

pub struct World {
    world: Vec<Box<dyn Shape>>
}