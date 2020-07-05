use serde_repr::*;

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        return match self {
            Tile::Wall => true,
            _ => false,
        };
    }
}
