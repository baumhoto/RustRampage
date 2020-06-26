use crate::consts::{WHITE, BLUE};

#[derive(Debug)]
pub struct FrameBuffer {
    width: usize,
    height: usize,
    pub(crate) pixels: Vec<u32>
}

pub fn new_framebuffer(width: usize, height: usize) -> FrameBuffer {
    let buffer: Vec<u32> = vec![WHITE; width * height];
        FrameBuffer{
            width: width,
            height: height,
            pixels: buffer
   }
}

impl FrameBuffer {
    pub fn get_color_at(&self, x: usize, y: usize) -> u32 {
        return self.pixels[y * self.width as usize + x];
    }

    pub fn set_color_at(&mut self, x: usize, y: usize, color: u32) {
        self.pixels[y * self.width as usize + x] = color;
    }
}