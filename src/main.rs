use std::{env, path};

use ggez;
use ggez::event;

mod state;
mod player;
mod player_renderer;
mod map;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub fn main() -> ggez::GameResult { 
    let resource_dir = if let Ok(manifest_dir)
        = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("graphics");
        path
    } else {
        path::PathBuf::from("./graphics")
    };
    let window_setup = ggez::conf::WindowSetup {
        title: "Firefly".to_owned(),
        samples: ggez::conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let window_mode = ggez::conf::WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };
    let cb = ggez::ContextBuilder::new("firefly", "Ben Brunton")
        .add_resource_path(resource_dir)
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut state::MainState::new(
        WINDOW_WIDTH, WINDOW_HEIGHT
    )?;
    let result = event::run(ctx, event_loop, state);
    match &result {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    };

    result
}
