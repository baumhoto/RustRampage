use crate::vector::Vector;
use crate::rect::Rect;

#[derive(Debug,Default)]
pub struct Player {
    pub position: Vector,
    pub velocity: Vector,
    pub radius: f64,
    pub speed: f64
}

impl Player {
    pub fn new(position: Vector) -> Self {
       Self {
           position,
           velocity: Vector { x: 0.0, y: 0.0 },
           radius: 0.5,
           speed: 2.0
       }
    }

    pub fn rect(&self) -> Rect {
        let half_size = Vector::new(self.radius, self.radius);
        return Rect::new(self.position - half_size, self.position + half_size);
    }
}