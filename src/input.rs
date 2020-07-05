use crate::rotation::Rotation;

#[derive(Debug, Default)]
pub struct Input {
    pub speed: f64,
    pub rotation: Rotation,
}

impl Input {
    pub fn new(speed: f64, rotation: Rotation) -> Self {
        Self { speed, rotation }
    }
}
