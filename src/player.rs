use crate::vector::Vector;
use crate::rect::Rect;
use crate::tilemap::Tilemap;

#[derive(Debug,Default)]
pub struct Player {
    pub position: Vector,
    pub velocity: Vector,
    pub direction: Vector,
    pub radius: f64,
    pub speed: f64
}

impl Player {
    pub fn new(position: Vector) -> Self {
       Self {
           position,
           velocity: Vector { x: 0.0, y: 0.0 },
           direction: Vector { x: 1.0, y: 0.0},
           radius: 0.25,
           speed: 2.0
       }
    }

    pub fn rect(&self) -> Rect {
        let half_size = Vector::new(self.radius, self.radius);
        return Rect::new(self.position - half_size, self.position + half_size);
    }

    pub fn is_intersecting(&self, map: &Tilemap) -> Option<Vector> {
        let rect = self.rect();
        let (min_x, max_x, min_y, max_y)
            = (rect.min.x as usize, rect.max.x as usize,
               rect.min.y as usize, rect.max.y as usize);
        let mut largest_intersection: Option<Vector> = None;
        for y in (min_y..=max_y).step_by(1) {
            for x in (min_x..=max_x).step_by(1) {
                if map.get_tile(x, y).is_wall() {
                    let wall_rect = Rect::new(Vector::new(x as f64, y as f64),
                                              Vector::new((x+1) as f64, (y+1)
                                                        as f64));
                    let intersection = rect.intersection(wall_rect);
                    if largest_intersection.is_none() ||
                        (intersection.is_some()
                            && intersection.unwrap().length() > largest_intersection.unwrap().length()) {
                        largest_intersection = intersection.clone();
                    }
                }
            }
        }
        return largest_intersection;
    }

}