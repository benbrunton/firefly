use ggez;
use ggez::{graphics, nalgebra as na};
use ggez::graphics::Text;
use crate::player::{
    Player, HorizontalDirection, VerticalDirection
};

pub fn draw(
    ctx: &mut ggez::Context,
    player: &Player
) -> ggez::GameResult {
    let image = graphics::Image::new(
        ctx, "/graphics/characters/Wizard.png"
    )?;
    let draw_param = get_player_draw_param(player);
    graphics::draw(
        ctx,
        &image,
        draw_param,
    )?;


    let text_frag = player.get_message();

    if text_frag.is_some() {
        let text = Text::new(text_frag.unwrap());
        let pos_x = 400 - 16;
        let pos_y = 300 - 16;
        let text_x = pos_x as f32 
            - (text.width(ctx) as f32 - 32.0) / 2.0;
        let text_y = pos_y as f32
            - text.height(ctx) as f32 - 2.0;
        let dst = na::Point2::new(text_x, text_y);
        let text_params = graphics::DrawParam::default()
            .dest(dst);

        graphics::draw(
            ctx,
            &text,
            text_params
        )?;
    }

    Ok(())
}

fn get_player_draw_param(player: &Player) -> graphics::DrawParam {
    let (h_direction, v_direction) = player.get_direction();
    let cycle = player.get_cycle();
    let (w, h) = (0.124, 0.5);
    let x_start = match h_direction {
        HorizontalDirection::Left => 0.125 * (3.0),
        _ => 0.0
    };

    let x = x_start + (0.125 * cycle);
    let y = match v_direction {
        VerticalDirection::Up => 0.5,
        _ => 0.0,
    };


    let rect = graphics::Rect::new(x, y, w, h);
    let xpos = 400 - 16;
    let ypos = 300 - 16;
    let dst = na::Point2::new(xpos as f32, ypos as f32);
    graphics::DrawParam::default()
        .dest(dst)
        .src(rect)
}
