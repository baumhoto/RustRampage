mod renderer;
mod consts;
mod framebuffer;

use minifb::{Key, Scale, Window, WindowOptions};
use consts::{WHITE, BLUE};
use framebuffer::{new_framebuffer};
use crate::renderer::new_renderer;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

fn main() {
    let mut renderer = new_renderer(WIDTH, HEIGHT);
    renderer.draw();

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&renderer.pixels(), WIDTH, HEIGHT).unwrap();
    }
}