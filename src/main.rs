mod tilemap;
mod tile;
mod rect;
mod world;
mod player;
mod vector;
mod renderer;
mod consts;
mod framebuffer;

use minifb::{Key, Scale, Window, WindowOptions};
use crate::renderer::Renderer;
use std::time::{Instant};
use crate::world::World;
use std::fs::File;
use std::io::BufReader;
use crate::tilemap::Tilemap;
use std::error::Error;

const WIDTH: usize = 320;
const HEIGHT: usize = 320;

fn main() {

    let tilemap = load_map().unwrap();

    let mut world = World::new(tilemap);
    //let mut world = World::new(Tilemap::new(Vec::new(), 8));
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Loading...",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
        .expect("Unable to Open Window");

    window.set_title("RustRampage");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut last_frame_time: f64 = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        renderer.draw(&world);
        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&renderer.pixels(), WIDTH, HEIGHT).unwrap();
        world.update(last_frame_time);
        renderer.clear_frame_buffer();
        last_frame_time = now.elapsed().as_secs_f64();
        //println!("{}", lastFrameTime)
    }
}

fn load_map() -> Result<Tilemap, Box<dyn Error>> {
    let f = File::open("src/map.json")?;
    let reader = BufReader::new(f);
    let tilemap = serde_json::from_reader(reader)?;
    Ok(tilemap)
}

