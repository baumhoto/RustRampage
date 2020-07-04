use serde_repr::*;

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        match self {
            Tile::Floor => false,
            Tile::Wall => true,
        }
    }
}
