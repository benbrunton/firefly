use ggez::{timer, Context, graphics};
use ggez::event::{self, KeyCode, KeyMods};
use crate::player::Player;
use crate::player_renderer;
use crate::map;

pub struct MainState {
    player: Player,
    map: map::Map
}

impl MainState {
    pub fn new() -> ggez::GameResult<MainState> {
        let player = Player::new();
        let map = map::Map::new();
        let s = MainState { player, map };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {

        while timer::check_update_time(ctx, 60) {
            self.player.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let coords = self.player.get_pos();
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let _ = self.map.draw(ctx, coords);
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
            KeyCode::Z => self.player.set_new_message(),
            // Quit if Ctrl+Q is pressed.
            KeyCode::Q => {
                if mods.contains(KeyMods::CTRL) {
                    println!("Terminating!");
                    event::quit(ctx);
                } 
            },
            _ => (),
        }
    }
    
    fn key_up_event(
        &mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods
    ) {
        match key {
            KeyCode::Up => self.player.end_move_up(),
            KeyCode::Down => self.player.end_move_down(),
            KeyCode::Left => self.player.end_move_left(),
            KeyCode::Right => self.player.end_move_right(),
            KeyCode::Z => self.player.cancel_message(),
            _ => ()
        }
    }
}

