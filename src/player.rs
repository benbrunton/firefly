use std::f32;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::map::Map;

// frame on which animation occurs
const FRAME_CYCLE: i32 = 7;
const WALKING_VELOCITY: f32 = 0.9;
const MESSAGE_DURATION: i32 = 140;
const NEGATIVE_PHRASES: [&str; 17] = [
    "Oh bloody hell!",
    "Fuck this shit!",
    "Piss in a bucket!",
    "Just fucking perfect",
    "Right... yeah... brilliant.",
    "Bollocks!",
    "Fucking, fucking, fucking, FUCK!",
    "Shitting balls!",
    "FML",
    "What's the fucking point?",
    "Why me?",
    "Oh for fuck's sake!",
    "FFS!",
    "I can't even.",
    "Jesus fucking Christ!",
    "Arse!",
    "Grumble, grumble, grumble..."
];

#[derive(Copy, Clone, PartialEq)]
pub enum HorizontalDirection {
    Left,
    Right
}

#[derive(Copy, Clone, PartialEq)]
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
    message: Option<String>,
    message_started: i32,
}

impl Player {
    pub fn new() -> Player {
        let diagonal_velocity = (f32::consts::PI/4.0)
            .sin() * WALKING_VELOCITY as f32;
        Player {
            pos_x: 10.0,
            pos_y: 300.0,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            horizontal_direction: HorizontalDirection::Right,
            vertical_direction: VerticalDirection::Down,
            cycle: 0.0,
            lifecycle: 0,
            diagonal_velocity,
            message: None,
            message_started: 0
        }
    }

    pub fn update(&mut self, map: &Map) {
        let mut moving = false;
        let mut pos_x = self.pos_x;
        let mut pos_y = self.pos_y;

        if self.moving_left {
            pos_x -= self.get_horizontal_velocity();
            self.horizontal_direction = HorizontalDirection::Left;
            moving = true;
        }

        if self.moving_right {
            pos_x += self.get_horizontal_velocity();
            self.horizontal_direction = HorizontalDirection::Right;
            moving = true;
        }

        if (self.moving_left || self.moving_right)
            && !self.moving_up {
            self.vertical_direction = VerticalDirection::Down;
        }

        if self.moving_up {
            pos_y -= self.get_vertical_velocity();
            self.vertical_direction = VerticalDirection::Up;
            moving = true;
        }

        if self.moving_down {
            pos_y += self.get_vertical_velocity();
            self.vertical_direction = VerticalDirection::Down;
            moving = true;
        }

        let tile = map.get_tile(pos_x, pos_y);
        match &*tile {
            "f" | "g" | "h" => (),
            _ => {
                self.pos_y = pos_y;
                self.pos_x = pos_x;
            }
        }
        

        if moving {
            self.update_cycle();
        } else {
            self.reset_cycle();
        }

        if self.lifecycle - self.message_started > MESSAGE_DURATION {
            self.cancel_message();
        }

        self.lifecycle += 1;
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

    pub fn set_new_message(&mut self) {
        if self.message.is_some() {
            return;
        }

        self.message_started = self.lifecycle;

        let mut rng = thread_rng();
        let phrase = NEGATIVE_PHRASES.choose(&mut rng)
            .unwrap_or(&"Bollocks!").to_string();
        self.message = Some(phrase);
    }

    fn cancel_message(&mut self) {
        self.message_started = 0;
        self.message = None;
    }

    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }

    // updates animation cycle
    fn update_cycle(&mut self) {
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
