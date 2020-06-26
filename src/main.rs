mod player;
mod vector;
mod renderer;
mod consts;
mod framebuffer;

use minifb::{Key, Scale, Window, WindowOptions};
use consts::{WHITE, BLUE};
use crate::vector::Vector;
use crate::player::{Player};
use crate::renderer::Renderer;
use std::borrow::{BorrowMut, Borrow};
use std::time::{Duration, Instant};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

fn main() {
    let mut player = Player::new(Vector{ x: 4.0, y: 4.0 });
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Loading...",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X32,
            ..WindowOptions::default()
        },
    )
        .expect("Unable to Open Window");

    window.set_title("RustRampage");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut lastFrameTime: f64 = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        renderer.draw(&player);
        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&renderer.pixels(), WIDTH, HEIGHT).unwrap();
        player.update(lastFrameTime);
        renderer.clearFrameBuffer();
        lastFrameTime = now.elapsed().as_secs_f64();
        //println!("{}", lastFrameTime)
    }
}