use crate::vector::Vector;

#[derive(Debug)]
pub struct Rect {
    pub min: Vector,
    pub max: Vector
}

impl Rect {
    pub fn new(min: Vector, max: Vector) -> Self {
        Self {
            min: min,
            max: max
        }
    }
}