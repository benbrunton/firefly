use ggez;
use ggez::{graphics, nalgebra as na};
use crate::player::{
    Player, HorizontalDirection, VerticalDirection
};

pub fn draw(
    ctx: &mut ggez::Context,
    player: &Player
) -> ggez::GameResult {
    let image = graphics::Image::new(
        ctx, "/characters/Rogue.png"
    )?;
    let (pos_x, pos_y) = player.get_pos();
    let (h_direction, v_direction) = player.get_direction();
    let (w, h) = (0.124, 0.5);
    let x = match h_direction {
        HorizontalDirection::Left => 0.125 * 3.0,
        _ => 0.0
    };
    let y = match v_direction {
        VerticalDirection::Up => 0.5,
        _ => 0.0,
    };


    let rect = graphics::Rect::new(x, y, w, h);
    let dst = na::Point2::new(pos_x, pos_y);
    let draw_param = graphics::DrawParam::default()
        .dest(dst)
        .src(rect);
    graphics::draw(
        ctx,
        &image,
        draw_param,
    )?;
    Ok(())
}
