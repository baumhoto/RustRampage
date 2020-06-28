use crate::player::Player;
use crate::vector::Vector;
use crate::tilemap::Tilemap;

pub struct World {
    pub map: Tilemap,
    pub player: Player
}

impl World {
    pub fn new(tilemap: Tilemap) -> Self {
        Self {
            map: tilemap,
            player: Player::new(Vector::new(4.0, 4.0))
        }
    }

    pub fn update(&mut self, time_step: f64) {
        let temp = Vector::new(self.player.velocity.x * time_step, self.player.velocity.y * time_step);
        self.player.position += temp;
        self.player.position.x = self.player.position.x % 8.0;
        self.player.position.y = self.player.position.y % 8.0;
        //println!("{}", self.position.x)
    }

    pub fn size(&self) -> Vector {
        return self.map.size()
    }
}