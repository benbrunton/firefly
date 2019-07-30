use ggez::Context;
use ggez::event::{self, KeyCode, KeyMods};
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

        self.player.update();
/*
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
*/

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let _ = player_renderer::draw(ctx, &self.player);

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        match key {
            KeyCode::Up => self.player.begin_move_up(),
            KeyCode::Down => self.player.begin_move_down(),
            KeyCode::Left => self.player.begin_move_left(),
            KeyCode::Right => self.player.begin_move_right(),
            // Quit if Shift+Ctrl+Q is pressed.
            KeyCode::Q => {
                if mods.contains(KeyMods::SHIFT) && mods.contains(KeyMods::CTRL) {
                    println!("Terminating!");
                    event::quit(ctx);
                } else if mods.contains(KeyMods::SHIFT) || mods.contains(KeyMods::CTRL) {
                    println!("You need to hold both Shift and Control to quit.");
                } else {
                    println!("Now you're not even trying!");
                }
            },
            _ => (),
        }
    }
    
    fn key_up_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods) {
        match key {
            KeyCode::Up => self.player.end_move_up(),
            KeyCode::Down => self.player.end_move_down(),
            KeyCode::Left => self.player.end_move_left(),
            KeyCode::Right => self.player.end_move_right(),
            _ => ()
        }
    }
}

