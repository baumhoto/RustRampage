use crate::vector::Vector;
use crate::rect::Rect;
use crate::tilemap::Tilemap;

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
           radius: 0.25,
           speed: 2.0
       }
    }

    pub fn rect(&self) -> Rect {
        let half_size = Vector::new(self.radius, self.radius);
        return Rect::new(self.position - half_size, self.position + half_size);
    }

    pub fn is_intersecting(&self, map: &Tilemap) -> bool {
        let rect = self.rect();
        let (minX, maxX, minY, maxY)
            = (rect.min.x as usize, rect.max.x as usize,
               rect.min.y as usize, rect.max.y as usize);
        for y in (minY..=maxY).step_by(1) {
            for x in (minX..=maxX).step_by(1) {
                if map.get_tile(x, y).is_wall() {
                    return true
                }
            }
        }
        return false;
    }

}