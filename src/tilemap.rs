use crate::ray::Ray;
use crate::thing::Thing;
use crate::tile::Tile;
use crate::vector::Vector;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Tilemap {
    tiles: Vec<Tile>,
    pub things: Vec<Thing>,
    pub width: usize,
}

impl Tilemap {
    pub fn height(&self) -> usize {
        return self.tiles.len() / self.width;
    }

    pub fn size(&self) -> Vector {
        return Vector::new(self.width as f64, self.height() as f64);
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        return &self.tiles[y * self.width + x];
    }

    fn tile(&self, position: Vector, direction: Vector) -> &Tile {
        let mut offset_x: f64 = 0.0;
        let mut offset_y: f64 = 0.0;
        if f64::floor(position.x) == position.x {
            offset_x = if direction.x > 0.0 { 0.0 } else { -1.0 };
        }
        if f64::floor(position.y) == position.y {
            offset_y = if direction.y > 0.0 { 0.0 } else { -1.0 };
        }

        let x = if (position.x + offset_x) >= 0.0 {
            (position.x + offset_x) as usize
        } else {
            0
        };
        let y = if (position.y + offset_y) >= 0.0 {
            (position.y + offset_y) as usize
        } else {
            0
        };
        //println!("{:?} {:?}", x, y);
        return &self.get_tile(x, y);
    }

    pub fn hit_test(&self, ray: Ray) -> Vector {
        let mut position = ray.origin;
        let slope = ray.direction.x / ray.direction.y;

        loop {
            let edge_distance_x: f64;
            let edge_distance_y: f64;

            if ray.direction.x > 0.0 {
                edge_distance_x = f64::floor(position.x) + 1.0 - position.x
            } else {
                edge_distance_x = f64::ceil(position.x) - 1.0 - position.x;
            }

            if ray.direction.y > 0.0 {
                edge_distance_y = f64::floor(position.y) + 1.0 - position.y;
            } else {
                edge_distance_y = f64::ceil(position.y) - 1.0 - position.y;
            }

            let step1 = Vector::new(edge_distance_x, edge_distance_x / slope);
            let step2 = Vector::new(edge_distance_y * slope, edge_distance_y);

            if step1.length() < step2.length() {
                position += step1
            } else {
                position += step2
            };

            if self.tile(position, ray.direction).is_wall() == true {
                break;
            }
        }

        return position;
    }
}
