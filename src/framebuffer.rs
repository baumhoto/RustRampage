use crate::rect::Rect;
use crate::consts::BLACK;
use crate::vector::Vector;

#[derive(Debug)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub(crate) pixels: Vec<u32>
}



impl FrameBuffer {
   pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![BLACK; width * height];
        Self{
            width: width,
            height: height,
            pixels: buffer
        }
    }

    pub fn set_color_at(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width as usize + x] = color;
        }
    }

    pub fn clear(&mut self) {
        for p in &mut self.pixels {
            *p = BLACK;
        }
    }

    pub fn fill(&mut self, rect: Rect, color: u32) {
        for y in ((rect.min.y as usize)..(rect.max.y as usize)).step_by(1) {
            for x in ((rect.min.x as usize)..(rect.max.x as usize)).step_by(1) {
                self.set_color_at(x,y, color)
            }
        }
    }

    pub fn draw_line(&mut self, from: Vector, to: Vector, color: u32) {
        let difference = to - from;
        let mut step: Vector;
        let step_count: usize;
        if f64::abs(difference.x) > f64::abs(difference.y) {
            step_count = f64::abs(difference.x).ceil() as usize;
            let sign = if difference.x > 0.0 { 1.0 } else { -1.0 };
            step = Vector {x: 1.0 , y: difference.y / difference.x};
            step.multiply(sign);
        } else {
            step_count = f64::abs(difference.y).ceil() as usize;
            let sign = if difference.y > 0.0 { 1.0 } else { -1.0 };
            step = Vector {x: difference.x / difference.y, y: 1.0 };
            step.multiply(sign);
        }

        let mut point = from;
        for _ in (0..step_count).step_by(1) {
            self.set_color_at(point.x as usize, point.y as usize, color);
            point += step;
        }
    }
}