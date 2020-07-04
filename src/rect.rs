use crate::vector::Vector;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vector,
    pub max: Vector,
}

impl Rect {
    pub fn new(min: Vector, max: Vector) -> Self {
        Self { min: min, max: max }
    }

    pub fn intersection(&self, with: Rect) -> Option<Vector> {
        let left = Vector::new(self.max.x - with.min.x, 0.0);
        if left.x <= 0.0 {
            return None;
        }
        let right = Vector::new(self.min.x - with.max.x, 0.0);
        if right.x >= 0.0 {
            return None;
        }
        let up = Vector::new(0.0, self.max.y - with.min.y);
        if up.y <= 0.0 {
            return None;
        }
        let down = Vector::new(0.0, self.min.y - with.max.y);
        if down.y >= 0.0 {
            return None;
        }
        let mut vectors = [left, right, up, down];
        vectors.sort_by_key(|k| OrderedFloat(k.length()));
        return vectors.first().cloned();
    }
}
