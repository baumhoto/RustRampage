use crate::consts::{BLUE, GREEN, RED, WHITE};
use crate::framebuffer::FrameBuffer;
use crate::ray::Ray;
use crate::rect::Rect;
use crate::vector::Vector;
use crate::world::World;

pub struct Renderer {
    pub(crate) frame_buffer: FrameBuffer,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            frame_buffer: FrameBuffer::new(width, height),
        };
    }

    pub fn draw(&mut self, world: &World) {
        let scale = self.frame_buffer.height as f64 / world.size().y;

        // Draw map
        for y in (0..world.map.height()).step_by(1) {
            for x in (0..world.map.width).step_by(1) {
                if world.map.get_tile(x, y).is_wall() {
                    let rect = Rect::new(
                        Vector::new(x as f64 * scale, y as f64 * scale),
                        Vector::new((x + 1) as f64 * scale, (y + 1) as f64 * scale),
                    );
                    self.frame_buffer.fill(rect, WHITE)
                }
            }
        }

        //Draw Player
        let mut rect = world.player.rect();
        rect.min.multiply(scale); // = multiply_vector(rect.min, scale);
        rect.max.multiply(scale); // = multiply_vector(rect.max, scale);
        self.frame_buffer.fill(rect, BLUE);

        // Draw line of sight
        // let ray = Ray::new(world.player.position, world.player.direction);
        // let end = world.map.hit_test(ray);
        // self.frame_buffer.draw_line(
        //     Vector::multiply_vector(world.player.position, scale),
        //     Vector::multiply_vector(end, scale),
        //     GREEN,
        // );

        // Draw view plane
        let focal_length = 1.0;
        let view_width = 1.0;
        let view_plane = Vector::multiply_vector(world.player.direction.orthogonal(), view_width);
        let view_center =
            world.player.position + Vector::multiply_vector(world.player.direction, focal_length);
        let view_start = view_center - Vector::divide_vector(view_plane, 2.0);
        let view_end = view_start + view_plane;
        self.frame_buffer.draw_line(
            Vector::multiply_vector(view_start, scale),
            Vector::multiply_vector(view_end, scale),
            RED,
        );

        // Cast rays
        let columns = 10;
        let step = Vector::divide_vector(view_plane, columns as f64);
        let mut column_position = view_start;

        for _ in 0..columns {
            let ray_direction = column_position - world.player.position;
            let view_plane_distance = ray_direction.length();
            let ray = Ray::new(
                world.player.position,
                Vector::divide_vector(ray_direction, view_plane_distance),
            );
            let end = world.map.hit_test(ray);
            self.frame_buffer.draw_line(
                Vector::multiply_vector(ray.origin, scale),
                Vector::multiply_vector(end, scale),
                GREEN,
            );
            column_position += step;
            //println!("{:?} {:?}", ray.origin, end)
        }
    }

    pub fn pixels(&self) -> &Vec<u32> {
        return &self.frame_buffer.pixels;
    }

    pub fn clear_frame_buffer(&mut self) {
        self.frame_buffer.clear()
    }
}
