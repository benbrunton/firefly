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
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
    cycle: f32,
    lifecycle: i32
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos_x: 0.0,
            pos_y: 0.0,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            cycle: 0.0,
            lifecycle: 0
        }
    }

    pub fn update(&mut self) {
        let mut moving = false;
        // if moving
        if self.moving_left {
            self.pos_x -= 0.5;
            moving = true;
        }

        if self.moving_right {
            self.pos_x += 0.5;
            moving = true;
        }

        if self.moving_up {
            self.pos_y -= 0.5;
            moving = true;
        }

        if self.moving_down {
            self.pos_y += 0.5;
            moving = true;
        }

        if moving {
            self.update_cycle();
        }
    }

    pub fn begin_move_left(&mut self) {
        self.moving_left = true;
    }

    pub fn end_move_left(&mut self) {
        self.moving_left = false;
    }

    pub fn begin_move_right(&mut self) {
        self.moving_right = true;
    }

    pub fn end_move_right(&mut self) {
        self.moving_right = false;
    }

    pub fn begin_move_up(&mut self) {
        self.moving_up = true;
    }

    pub fn end_move_up(&mut self) {
        self.moving_up = false;
    }

    pub fn begin_move_down(&mut self) {
        self.moving_down = true;
    }

    pub fn end_move_down(&mut self) {
        self.moving_down = false;
    }


    pub fn get_pos(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    pub fn get_direction(&self) -> (HorizontalDirection, VerticalDirection) {
        let horizontal = if self.moving_left {
            HorizontalDirection::Left
        } else {
            HorizontalDirection::Right
        };

        let vertical = if self.moving_up {
            VerticalDirection::Up
        } else {
            VerticalDirection::Down
        };

        (horizontal, vertical)
    }

    pub fn get_cycle(&self) -> f32 {
        self.cycle
    }

    fn update_cycle(&mut self) {
        self.lifecycle += 1;
        if self.lifecycle % 5 == 0 {
            self.cycle += 1.0;
        }

        if self.cycle > 2.0 {
            self.cycle = 0.0;
        }

    }
}
