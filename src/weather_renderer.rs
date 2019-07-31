use ggez;
use ggez::{graphics, nalgebra as na};
use ggez::graphics::spritebatch::SpriteBatch;

static IMAGE_LIST: [&str;4] = [
    "/effects/rain_drops-01.png",
    "/effects/rain_drops-02.png",
    "/effects/rain_drops-03.png",
    "/effects/rain_drops-04.png"
];

const TILE_WIDTH: f32 = 256.0;
const TILE_HEIGHT: f32 = 240.0;
const ANIMATION_RATE: i32 = 6;

pub struct WeatherRenderer{
    lifecycle: i32,
    image_index: usize
}

impl WeatherRenderer {
    pub fn new() -> WeatherRenderer {
        WeatherRenderer{
            lifecycle: 0,
            image_index: 0
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> ggez::GameResult {
        self.lifecycle += 1;
        let image_index = self.get_image_index();
        let image = graphics::Image::new(
            ctx, IMAGE_LIST[image_index]
        )?;
        
        let mut spritebatch = SpriteBatch::new(image);

        let mut i = 0;
        while i < 3 {
            let mut j = 0;
            while j < 4 {
                let xpos = j as f32 * TILE_WIDTH;
                let ypos = i as f32 * TILE_HEIGHT;
                let dst = na::Point2::new(xpos, ypos);
                let params = graphics::DrawParam::default()
                    .dest(dst);
                spritebatch.add(
                    params
                );

                j += 1;
            }
            i += 1;
        }

        graphics::draw(
            ctx,
            &spritebatch,
            graphics::DrawParam::default()
        )
    }

    fn get_image_index(&mut self) -> usize {
        if self.lifecycle % ANIMATION_RATE == 0 {
            self.image_index += 1;
        }

        if self.image_index > 3 {
            self.image_index = 0;
        }

        self.image_index
    }
}


