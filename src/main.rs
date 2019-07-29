use std::{env, path};

use ggez;
use ggez::event;

mod state;
mod player;
mod player_renderer;

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
    let cb = ggez::ContextBuilder::new("firefly", "Ben Brunton")
        .add_resource_path(resource_dir)
        .window_setup(window_setup);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut state::MainState::new()?;
    let result = event::run(ctx, event_loop, state);
    match &result {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    };

    result
}
