use crate::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }
}
