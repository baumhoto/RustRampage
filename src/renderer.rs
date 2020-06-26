use crate::framebuffer::{FrameBuffer};
use crate::consts::BLUE;
use crate::player::Player;

pub struct Renderer {
   pub(crate) frameBuffer : FrameBuffer
}


impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self{frameBuffer: FrameBuffer::new(width, height)};
    }

    pub fn draw(&mut self, player: &Player) {
       self.frameBuffer.set_color_at(player.position.x as usize,player.position.y as usize,BLUE)
    }

    pub fn pixels(&self) -> &Vec<u32> {
        return &self.frameBuffer.pixels;
    }

    pub fn clearFrameBuffer(&mut self) {
        self.frameBuffer.clear()
    }
}