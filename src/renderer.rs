use crate::framebuffer::{FrameBuffer, new_framebuffer};
use crate::consts::BLUE;

pub struct Renderer {
   pub(crate) frameBuffer : FrameBuffer
}

pub fn new_renderer(width: usize, height: usize) -> Renderer {
  return Renderer{frameBuffer: new_framebuffer(width, height)};
}

impl Renderer {
    pub fn draw(&mut self) {
       self.frameBuffer.set_color_at(0, 0, BLUE)
    }

    pub fn pixels(&self) -> &Vec<u32> {
        return &self.frameBuffer.pixels;
    }
}