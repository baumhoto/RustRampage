use crate::framebuffer::{FrameBuffer};
use crate::consts::BLUE;
use crate::player::Player;
use crate::world::World;
use crate::vector::multiply_vector;

pub struct Renderer {
   pub(crate) frameBuffer : FrameBuffer
}


impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self{frameBuffer: FrameBuffer::new(width, height)};
    }

    pub fn draw(&mut self, world: &World) {
        let scale = self.frameBuffer.height as f64 / world.size.y;

        //Draw Player
        let mut rect = world.player.rect();
        rect.min.multiply(scale);   // = multiply_vector(rect.min, scale);
        rect.max.multiply(scale);  // = multiply_vector(rect.max, scale);
        self.frameBuffer.fill(rect, BLUE);
    }

    pub fn pixels(&self) -> &Vec<u32> {
        return &self.frameBuffer.pixels;
    }

    pub fn clear_frame_buffer(&mut self) {
        self.frameBuffer.clear()
    }
}