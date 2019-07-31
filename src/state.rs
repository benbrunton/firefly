use ggez::{timer, Context, graphics};
use ggez::event::{self, KeyCode, KeyMods};
use crate::player::Player;
use crate::player_renderer;
use crate::map;
use crate::weather_renderer::WeatherRenderer;

pub struct MainState {
    player: Player,
    map: map::Map,
    weather_renderer: WeatherRenderer
}

impl MainState {
    pub fn new(
        window_width: f32, window_height: f32
    ) -> ggez::GameResult<MainState> {
        let player = Player::new();
        let map = map::Map::new(window_width, window_height);
        let weather_renderer = WeatherRenderer::new();
        let s = MainState { player, map, weather_renderer };
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
        let _ = self.weather_renderer.draw(ctx);

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
            _ => ()
        }
    }
}

