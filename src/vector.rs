use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,y
        }
    }

    pub fn multiply(&mut self, multiplier: f64)  {
        self.x *= multiplier;
        self.y *= multiplier;
    }

    pub fn multiply_vector(vector: Vector, multiplier: f64) -> Vector {
        return Vector::new(vector.x * multiplier, vector.y * multiplier);
    }

    pub fn divide_vector(vector: Vector, divisor: f64) -> Vector {
        return Vector::new(vector.x / divisor, vector.y / divisor);
    }

    pub fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt()
    }
}


impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
       Vector{x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector{x: self.x - other.x, y: self.y - other.y}
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y};
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{x: self.x - other.x, y: self.y - other.y};
    }
}



