use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign};

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

    pub fn multiply(&mut self, multiplier: f64) {
        self.x *= multiplier;
        self.y *= multiplier;
    }
}

pub fn multiply_vector(vector: Vector, multiplier: f64) -> Vector {
    return Vector::new(vector.x * multiplier, vector.y * multiplier);
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

//impl Div for Vector {
//     type Output = Vector;
//
//     fn div(self, other: f64) -> Vector {
//         Vector{x: self.x / other, y: self.y / other}
//     }
// }
//
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

// impl DivAssign for Vector {
//     fn div_assign(&mut self, other: f64) {
//         *self = Self{x: self.x / other, y: self.y / other};
//     }
// }
//
// impl MulAssign for Vector {
//     fn mul_assign(&mut self, other: f64)  {
//         *self = Self{x: self.x * other, y: self.y * other};
//     }
// }


