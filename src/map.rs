use ggez;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::{graphics, nalgebra as na};

const TILE_WIDTH:f32 = 32.0;

pub struct Map {
    window_width: f32,
    window_height: f32
}

impl Map {
    pub fn new(window_width: f32, window_height: f32) -> Map {
        Map{ window_width, window_height }
    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        (player_x_pos, player_y_pos): (f32, f32)
    ) -> ggez::GameResult {
        let xpos = self.window_width / 2.0
            - TILE_WIDTH / 2.0 - player_x_pos;
        let ypos = self.window_height / 2.0
            - TILE_WIDTH / 2.0 - player_y_pos;

        let image = graphics::Image::new(
            ctx, "/terrain/TerrainTile.png"
        ).expect("unable to load terrain tiles");
        let mut spritebatch = SpriteBatch::new(image);

        let offset_x = xpos + TILE_WIDTH
            + (player_x_pos % TILE_WIDTH);
        let offset_y = ypos + TILE_WIDTH
            + (player_y_pos % TILE_WIDTH);

        // window defaults 800x600
        // which is 25 x 18.75 tiles
        // but we need to have another set of tiles around
        let mut i = 0;
        while i < 21 {
            let mut j = 0;
            while j < 27 {
                let x = j as f32 * TILE_WIDTH - offset_x;
                let y = i as f32 * TILE_WIDTH - offset_y;
                let dst = na::Point2::new(x, y);
                let rect = graphics::Rect::new(0.0, 0.0, 0.1, 0.1);
                let draw_param = graphics::DrawParam::default()
                    .dest(dst)
                    .src(rect);

                spritebatch.add(
                    draw_param,
                );
                j += 1;
            }
            i += 1;
        }

        let dst = na::Point2::new(xpos, ypos);
        graphics::draw(
            ctx,
            &spritebatch,
            graphics::DrawParam::default()
                .dest(dst)
        )
    }
}

