use crate::framebuffer::{FrameBuffer};
use crate::consts::{BLUE, WHITE, GREEN};
use crate::world::World;
use crate::rect::Rect;
use crate::vector::Vector;

pub struct Renderer {
   pub(crate) frame_buffer: FrameBuffer
}


impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self{ frame_buffer: FrameBuffer::new(width, height)};
    }

    pub fn draw(&mut self, world: &World) {
        let scale = self.frame_buffer.height as f64 / world.size().y;

        // Draw map
        for y in (0..world.map.height()).step_by(1) {
            for x in (0..world.map.width).step_by(1) {
                if world.map.get_tile(x, y).is_wall() {
                    let rect = Rect::new(
                        Vector::new(x as f64 * scale, y as f64 * scale),
                        Vector::new((x + 1) as f64 * scale, (y+1) as f64 * scale)
                    );
                    self.frame_buffer.fill(rect, WHITE)
                }
            }
        }

        //Draw Player
        let mut rect = world.player.rect();
        rect.min.multiply(scale);   // = multiply_vector(rect.min, scale);
        rect.max.multiply(scale);  // = multiply_vector(rect.max, scale);
        self.frame_buffer.fill(rect, BLUE);

        // Draw line of sight
        let end = world.player.position + Vector::multiply_vector(world.player.direction, 100.0);
        self.frame_buffer.draw_line(Vector::multiply_vector(world.player.position, scale),
                                    Vector::multiply_vector(end, scale), GREEN);
    }

    pub fn pixels(&self) -> &Vec<u32> {
        return &self.frame_buffer.pixels;
    }

    pub fn clear_frame_buffer(&mut self) {
        self.frame_buffer.clear()
    }
}