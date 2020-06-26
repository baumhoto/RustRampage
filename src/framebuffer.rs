use crate::consts::{WHITE, BLUE};
use crate::rect::Rect;

#[derive(Debug)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub(crate) pixels: Vec<u32>
}



impl FrameBuffer {
   pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![WHITE; width * height];
        Self{
            width: width,
            height: height,
            pixels: buffer
        }
    }

    pub fn get_color_at(&self, x: usize, y: usize) -> u32 {
        return self.pixels[y * self.width as usize + x];
    }

    pub fn set_color_at(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width as usize + x] = color;
        }
    }

    pub fn clear(&mut self) {
        for p in &mut self.pixels {
            *p = WHITE;
        }
    }

    pub fn fill(&mut self, rect: Rect, color: u32) {
        for y in ((rect.min.y as usize)..(rect.max.y as usize)).step_by(1) {
            for x in ((rect.min.x as usize)..(rect.max.x as usize)).step_by(1) {
                self.set_color_at(x,y, color)
            }
        }
    }
}