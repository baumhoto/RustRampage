use crate::tile::Tile;
use crate::vector::Vector;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Tilemap {
    tiles: Vec<Tile>,
    pub width: usize
}

impl Tilemap {
    pub fn new(tiles: Vec<Tile>, width: usize) -> Self {
        Self {
            tiles,
            width
        }
    }

    pub fn height(&self) -> usize {
       return self.tiles.len() / self.width
    }

    pub fn size(&self) -> Vector {
       return Vector::new(self.width as f64, self.height() as f64)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        return &self.tiles[y * self.width + x];
    }
}