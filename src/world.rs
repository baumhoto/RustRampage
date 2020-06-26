use crate::player::Player;
use crate::vector::Vector;

pub struct World {
    pub size: Vector,
    pub player: Player
}

impl World {
    pub fn new() -> Self {
        Self {
            size: Vector::new(8.0,  8.0),
            player: Player::new(Vector::new(4.0, 4.0))
        }
    }

    pub fn update(&mut self, timeStep: f64) {
        let temp = Vector::new(self.player.velocity.x * timeStep, self.player.velocity.y * timeStep);
        self.player.position += temp;
        self.player.position.x = self.player.position.x % 8.0;
        self.player.position.y = self.player.position.y % 8.0;
        //println!("{}", self.position.x)
    }

}