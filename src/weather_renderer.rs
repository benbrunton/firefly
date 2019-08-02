use ggez;
use ggez::{graphics, nalgebra as na};
use ggez::graphics::spritebatch::SpriteBatch;
use rand::Rng;

static IMAGE_LIST: [&str;4] = [
    "/effects/rain_drops-01.png",
    "/effects/rain_drops-02.png",
    "/effects/rain_drops-03.png",
    "/effects/rain_drops-04.png"
];

const TILE_WIDTH: f32 = 256.0;
const TILE_HEIGHT: f32 = 240.0;
const ANIMATION_RATE: i32 = 6;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub struct WeatherRenderer{
    lifecycle: i32,
    image_index: usize,
    lightning: (i32, f32, f32), // time left, colour, alpha
}

impl WeatherRenderer {
    pub fn new() -> WeatherRenderer {
        WeatherRenderer{
            lifecycle: 0,
            image_index: 0,
            lightning: (0, 0.0, 0.0),
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
        )?;

        if self.lightning.0 > 0 {
            self.draw_lightning(
                ctx,
                self.lightning.1,
                self.lightning.2
            )?;
            self.lightning.0 -= 1;
        } else {
            self.generate_lightning();
        }

        Ok(())
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

    fn generate_lightning(&mut self) {
        let mut rng = rand::thread_rng(); 
        let random_num = rng.gen_range(0, 1000);

        if random_num < 1 {
            self.lightning = (12, 0.8, 0.6);
        } else if random_num < 2 {
            self.lightning = (30, 0.0, 0.3);
        } else if random_num < 3 {
            self.lightning = (5, 1.0, 0.7);
        } else if random_num < 4 {
            self.lightning = (80, 0.2, 0.2);
        }
    }

    fn draw_lightning(
        &mut self,
        ctx: &mut ggez::Context,
        grey: f32,
        alpha: f32,
    ) -> ggez::GameResult{
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT
            ),
            graphics::Color::new(grey, grey, grey, alpha)
        )?;

        graphics::draw(
            ctx,
            &rect,
            graphics::DrawParam::default()
        )
    }
}


