// TODO: turn this into a union
#[derive(Copy, Debug)]
pub struct Color {

}

impl Clone for Color {
    fn clone(&self) -> Color {
        *self
    }
}