use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{graphics, nalgebra as na, timer};
use ggez::input::keyboard;

pub struct MainState {
    pos_x: f32,
}

impl MainState {
    pub fn new() -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.pos_x += 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.pos_x -= 0.5;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

