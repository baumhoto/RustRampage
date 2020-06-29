use crate::player::Player;
use crate::vector::Vector;
use crate::tilemap::Tilemap;
use crate::thing::Thing;
use std::ptr::null;

pub struct World {
    pub map: Tilemap,
    pub player: Player
}

impl World {
    pub fn new(tilemap: Tilemap) -> Self {
        let mut player : Option<Player> = None;

        for y in (0..tilemap.height()).step_by(1) {
            for x in (0..tilemap.width).step_by(1) {
                let position = Vector::new(x as f64 +0.5, y as f64 + 0.5);
                let thing = &tilemap.things[y * tilemap.width + x];
                match thing {
                    Thing::Player => player = Some(Player::new(position)),
                    Thing::Nothing => Default::default()
                };
            }
        }

        let result = if player.is_some()  { player.unwrap() } else { Default::default() };

        Self {
            map: tilemap,
            player: result
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