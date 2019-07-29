#[derive(Copy, Clone)]
pub enum HorizontalDirection {
    Left,
    Right
}

#[derive(Copy, Clone)]
pub enum VerticalDirection {
    Up,
    Down
}

pub struct Player {
    pos_x: f32,
    pos_y: f32,
    horizontal: HorizontalDirection,
    vertical: VerticalDirection
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x: 0.0,
            pos_y: 0.0,
            horizontal: HorizontalDirection::Left,
            vertical: VerticalDirection::Down,
        }
    }

    pub fn move_left(&mut self) {
        self.pos_x -= 0.5;
        self.horizontal = HorizontalDirection::Left;
    }

    pub fn move_right(&mut self) {
        self.pos_x += 0.5;
        self.horizontal = HorizontalDirection::Right;
    }

    pub fn move_forward(&mut self) {
        self.pos_y -= 0.5;
        self.vertical = VerticalDirection::Up;
    }

    pub fn move_backward(&mut self) {
        self.pos_y += 0.5;
        self.vertical = VerticalDirection::Down;
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    pub fn get_direction(&self) -> (HorizontalDirection, VerticalDirection) {
        (self.horizontal.clone(), self.vertical.clone())
    }

}
