use ggez;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::{graphics, nalgebra as na};

pub struct Map {
}

impl Map {
    pub fn new() -> Map {

        Map {
        }

    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        (player_x_pos, player_y_pos): (f32, f32)
    ) -> ggez::GameResult {
        let xpos = 400.0 - 16.0 - player_x_pos;
        let ypos = 300.0 - 16.0 - player_y_pos;

        let image = graphics::Image::new(
            ctx, "/terrain/TerrainTile.png"
        ).expect("unable to load terrain tiles");
        let mut spritebatch = SpriteBatch::new(image);

        let offset_x = xpos + (player_x_pos % 32.0) + 32.0;
        let offset_y = ypos + (player_y_pos % 32.0) + 32.0;

        // window defaults 800x600
        // which is 25 x 18.75 tiles
        // but we need to have another set of tiles around
        let mut i = 0;
        while i < 21 {
            let mut j = 0;
            while j < 27 {
                let x = j as f32 * 32.0 - offset_x;
                let y = i as f32 * 32.0 - offset_y;
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

