mod consts;
mod framebuffer;
mod input;
mod player;
mod ray;
mod rect;
mod renderer;
mod rotation;
mod thing;
mod tile;
mod tilemap;
mod vector;
mod world;

use crate::input::Input;
use crate::renderer::Renderer;
use crate::rotation::Rotation;
use crate::tilemap::Tilemap;
use crate::vector::Vector;
use crate::world::World;
use minifb::{Key, Scale, Window, WindowOptions};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const MAX_TIMESTEP: f64 = 1.0 / 20.0;
const WORLD_TIMESTEP: f64 = 1.0 / 120.0;

fn main() {
    let tilemap = load_map().unwrap();
    //println!("{:?}", tilemap);
    let mut world = World::new(tilemap);
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
        let input = handle_input(&window, world.player.turning_speed);

        let timestep = f64::min(MAX_TIMESTEP, last_frame_time);
        let world_steps = (timestep / WORLD_TIMESTEP).ceil();

        for _ in (0..world_steps as i32).step_by(1) {
            world.update(timestep, &input);
        }

        renderer.draw(&world);

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&renderer.pixels(), WIDTH, HEIGHT)
            .unwrap();
        last_frame_time = now.elapsed().as_secs_f64();

        renderer.clear_frame_buffer();
        //println!("{}", lastFrameTime)
    }
}

fn load_map() -> Result<Tilemap, Box<dyn Error>> {
    let f = File::open("src/map.json")?;
    let reader = BufReader::new(f);
    let tilemap = serde_json::from_reader(reader)?;
    Ok(tilemap)
}

fn handle_input(window: &Window, turing_speed: f64) -> Input {
    let mut input_vector = Vector::default();
    if window.is_key_down(Key::Up) {
        input_vector.y = -1.0
    } else if window.is_key_down(Key::Down) {
        input_vector.y = 1.0
    } else if window.is_key_down(Key::Left) {
        input_vector.x = -1.0
    } else if window.is_key_down(Key::Right) {
        input_vector.x = 1.0
    }

    let rotation = input_vector.x * turing_speed * WORLD_TIMESTEP;

    let input = Input::new(
        -input_vector.y,
        Rotation::new(f64::sin(rotation), f64::cos(rotation)),
    );
    return input;
}
