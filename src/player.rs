use crate::vector::Vector;
use std::borrow::Borrow;

pub struct Player {
    pub position: Vector,
    pub velocity: Vector
}

impl Player {
    pub fn new(position: Vector) -> Self {
       Self {
           position: position,
           velocity: Vector { x: 1.0, y: 1.0 }
       }
    }

    pub fn update(&mut self, timeStep: f64) {
        let temp = Vector::new(self.velocity.x * timeStep, self.velocity.y * timeStep);
        self.position += temp;
        self.position.x = self.position.x % 8.0;
        self.position.y = self.position.y % 8.0;
        //println!("{}", self.position.x)
    }
}