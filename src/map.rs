use std::collections::HashMap;
use ggez;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::{graphics, nalgebra as na};
use noise::{Perlin, NoiseFn, Seedable};

const TILE_WIDTH:f32 = 32.0;
const TILE_FRACTION:f32 = 0.1;
const TILE_TYPE_KEYS:[&str; 8] = [
    "h",
    "g",
    "f",
    "e",
    "d",
    "c",
    "b",
    "a",
];

pub struct Map {
    window_width: f32,
    window_height: f32,
    stored_tiles: HashMap<String, String>,
    tile_types: HashMap<String, f32>,
    noise: Perlin
}

impl Map {
    pub fn new(window_width: f32, window_height: f32) -> Map {
        let stored_tiles = HashMap::new();
        let tile_types = [
            ("a".to_string(), TILE_FRACTION * 0.0),
            ("b".to_string(), TILE_FRACTION * 1.0),
            ("c".to_string(), TILE_FRACTION * 2.0),
            ("d".to_string(), TILE_FRACTION * 3.0),
            ("e".to_string(), TILE_FRACTION * 4.0),
            ("f".to_string(), TILE_FRACTION * 5.0),
            ("g".to_string(), TILE_FRACTION * 6.0),
            ("h".to_string(), TILE_FRACTION * 7.0),
        ].iter().cloned().collect();

        let noise = Perlin::new().set_seed(5);


        Map{
            window_width,
            window_height,
            stored_tiles,
            tile_types,
            noise
        }
    }

    pub fn draw(
        &mut self,
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

        let offset_x = (xpos + TILE_WIDTH
            + (player_x_pos % TILE_WIDTH)).round();
        let offset_y = (ypos + TILE_WIDTH
            + (player_y_pos % TILE_WIDTH)).round();

        // window defaults 800x600
        // which is 25 x 18.75 tiles
        // but we need to have another set of tiles around
        let mut i = 0;
        while i < 21 {
            let mut j = 0;
            while j < 27 {
                let x = j as f32 * TILE_WIDTH - offset_x;
                let y = i as f32 * TILE_WIDTH - offset_y;

                let key = format!("{}/{}", x, y);
                let tile_type = match self.stored_tiles.get(&key) {
                    Some(v) => v.clone(),
                    None    => {
                        let tile_t = self.get_tile_type(x, y);

                        self.stored_tiles.insert(
                            key, tile_t.clone()
                        );

                        tile_t.clone()
                    }
                };

                let x_val = self.tile_types.get(&tile_type)
                    .unwrap_or(&0.0)
                    .clone();
                let dst = na::Point2::new(x, y);
                let rect = graphics::Rect::new(
                    x_val, 0.0, 0.1, 0.1
                );
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

    fn get_tile_type(&self, x: f32, y: f32) -> String {
        let mut p = (self.noise.get(
            [
                (x / 1000.0) as f64, 
                (y / 1000.0) as f64
            ]
        ) * 20.0).abs() as usize;

        if p >= TILE_TYPE_KEYS.len() {
            p = TILE_TYPE_KEYS.len() - 1;
        }

        let tile_t = TILE_TYPE_KEYS[p];
        tile_t.to_string()

    }
}

