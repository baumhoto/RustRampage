use crate::vector::Vector;
use std::borrow::Borrow;
use crate::rect::Rect;

pub struct Player {
    pub position: Vector,
    pub velocity: Vector,
    pub radius: f64
}

impl Player {
    pub fn new(position: Vector) -> Self {
       Self {
           position: position,
           velocity: Vector { x: 1.0, y: 1.0 },
           radius: 0.5
       }
    }

    pub fn rect(&self) -> Rect {
        let halfSize = Vector::new(self.radius, self.radius);
        return Rect::new(self.position - halfSize, self.position + halfSize);
    }
}