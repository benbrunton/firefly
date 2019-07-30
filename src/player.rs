use std::f32;

// frame on which animation occurs
const FRAME_CYCLE: i32 = 10;
const WALKING_VELOCITY: f32 = 0.5;

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
    horizontal_direction: HorizontalDirection,
    vertical_direction: VerticalDirection,
    cycle: f32,
    lifecycle: i32,
    diagonal_velocity: f32,
}

impl Player {
    pub fn new() -> Player {
        let diagonal_velocity = (f32::consts::PI/4.0)
            .sin() * WALKING_VELOCITY as f32;
        Player {
            pos_x: 0.0,
            pos_y: 0.0,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            horizontal_direction: HorizontalDirection::Left,
            vertical_direction: VerticalDirection::Down,
            cycle: 0.0,
            lifecycle: 0,
            diagonal_velocity
        }
    }

    pub fn update(&mut self) {
        let mut moving = false;

        if self.moving_left {
            self.pos_x -= self.get_horizontal_velocity();
            self.horizontal_direction = HorizontalDirection::Left;
            moving = true;
        }

        if self.moving_right {
            self.pos_x += self.get_horizontal_velocity();
            self.horizontal_direction = HorizontalDirection::Right;
            moving = true;
        }

        if (self.moving_left || self.moving_right)
            && !self.moving_up {
            self.vertical_direction = VerticalDirection::Down;
        }

        if self.moving_up {
            self.pos_y -= self.get_vertical_velocity();
            self.vertical_direction = VerticalDirection::Up;
            moving = true;
        }

        if self.moving_down {
            self.pos_y += self.get_vertical_velocity();
            self.vertical_direction = VerticalDirection::Down;
            moving = true;
        }

        if moving {
            self.update_cycle();
        } else {
            self.reset_cycle();
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

        (self.horizontal_direction, self.vertical_direction)
    }

    pub fn get_cycle(&self) -> f32 {
        self.cycle
    }

    // updates animation cycle
    fn update_cycle(&mut self) {
        self.lifecycle += 1;
        if self.lifecycle % FRAME_CYCLE == 0 {
            self.cycle += 1.0;
        }

        if self.cycle > 2.0 {
            self.cycle = 0.0;
        }

    }

    fn reset_cycle(&mut self) {
        self.cycle = 0.0;
    }

    fn get_horizontal_velocity(&self) -> f32 {
        if self.moving_up || self.moving_down {
            self.diagonal_velocity    
        } else {
            WALKING_VELOCITY
        }
    }

    fn get_vertical_velocity(&self) -> f32 {
        if self.moving_left || self.moving_right {
            self.diagonal_velocity    
        } else {
            WALKING_VELOCITY
        }
    }

}
