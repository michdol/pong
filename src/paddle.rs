use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::drawing::{draw_block};
use crate::game::{Movement, SPEED};

pub const PADDLE_LENGTH: i32 = 5;
const PADDLE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub movement_direction: Movement,
    length: i32,
    counter: i32,
    speed: i32
}

impl Paddle {
    pub fn new(x: i32, y: i32) -> Paddle {
        Paddle {
            x: x,
            y: y,
            movement_direction: Movement::Right,
            length: PADDLE_LENGTH,
            counter: 0,
            speed: 0
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Paddle: 4 blocks long
        for i in 0..self.length {
            draw_block(PADDLE_COLOR, self.x + i, self.y, con, g);
        }
    }

    pub fn update(&mut self) {
        // Slow down the paddle, update it every second method call.
        if self.counter == 1 {
            self.counter = 0;
            return
        }
        self.counter += 1;
        match self.movement_direction {
            Movement::Left => {
                self.x -= self.speed;
            },
            Movement::Right => {
                self.x += self.speed;
            },
            _ => {}
        }
    }

    pub fn set_direction(&mut self, dir: Option<Movement>) {
        match dir {
            Some(d) => {
                self.speed = SPEED;
                self.movement_direction = d;
            }
            None => {}
        }
    }

    pub fn stop(&mut self) {
        self.speed = 0;
    }

    pub fn detect_paddle_wall_collision(&mut self, width: i32) {
        match self.movement_direction {
            Movement::Right => {
                if self.x + SPEED >= width - PADDLE_LENGTH {
                    self.speed = 0;
                }
            },
            Movement::Left => {
                if self.x - SPEED <= 0 {
                    self.speed = 0;
                }
            },
            _ => {}
        }
    }
}


