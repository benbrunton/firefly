use ggez;
use ggez::event;

mod state;

pub fn main() -> ggez::GameResult { 
    let window_setup = ggez::conf::WindowSetup {
        title: "Firefly".to_owned(),
        samples: ggez::conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };
    let cb = ggez::ContextBuilder::new("firefly", "Ben Brunton")
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
