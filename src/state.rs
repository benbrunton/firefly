use ggez::event::{self, KeyCode};
use ggez::graphics;
use ggez::input::keyboard;

use crate::player::Player;
use crate::player_renderer;

pub struct MainState {
    player: Player,
}

impl MainState {
    pub fn new() -> ggez::GameResult<MainState> {
        let player = Player::new();
        let s = MainState { player };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.player.move_right();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.player.move_left();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.player.move_forward();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player.move_backward();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let _ = player_renderer::draw(ctx, &self.player);

        graphics::present(ctx)?;
        Ok(())
    }
}

