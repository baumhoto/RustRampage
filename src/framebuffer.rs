use crate::consts::{WHITE, BLUE};

#[derive(Debug)]
pub struct FrameBuffer {
    width: usize,
    height: usize,
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
}