use crate::vector::Vector;

#[derive(Debug,Default)]
pub struct Input {
    pub velocity: Vector
}

impl Input {
    pub fn new(velocity: Vector) -> Self {
        Self {
            velocity
        }
    }
}