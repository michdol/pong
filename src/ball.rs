use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::drawing::{draw_block};
use crate::game::{Movement, SPEED};
use crate::paddle::{Paddle};

const BALL_COLOR: Color = [0.984, 0.537, 0.56, 1.0];

const FAST_SPEED_FACTOR: i32 = 10;
const DEFAULT_SPEED_FACTOR: i32 = 15;
const SLOW_SPEED_FACTOR: i32 = 25;
const SPEED_ZERO: i32 = 99999999;

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_direction: Movement,
    pub y_direction: Movement,
    x_speed_factor: i32,
    y_speed_factor: i32,
    counter_x: i32,
    counter_y: i32,
}

impl Ball {
    pub fn new (x: i32, y: i32) -> Ball {
        Ball {
            x: x,
            y: y,
            x_direction: Movement::Left,
            y_direction: Movement::Down,
            x_speed_factor: SPEED_ZERO,
            y_speed_factor: SLOW_SPEED_FACTOR,
            counter_x: 0,
            counter_y: 0,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.x, self.y, con, g);
    }

    pub fn update(&mut self) {
        self.update_x();
        self.update_y();
    }

    pub fn update_x(&mut self) {
        if self.x_speed_factor == SPEED_ZERO {
            return
        }
        if self.counter_x != self.x_speed_factor {
            self.counter_x += 1;
            return
        }
        self.counter_x = 0;
        match self.x_direction {
            Movement::Right => self.x += 1,
            Movement::Left => self.x -= 1,
            _ => {}
        }
    }

    pub fn update_y(&mut self) {
        if self.y_speed_factor == SPEED_ZERO {
            return
        }
        if self.counter_y != self.y_speed_factor {
            self.counter_y += 1;
            return
        }
        self.counter_y = 0;
        match self.y_direction {
            Movement::Up => self.y -= 1,
            Movement::Down => self.y += 1,
            _ => {}
        }
    }

    pub fn detect_ball_wall_collision(&mut self, width: i32) {
        match self.x_direction {
            Movement::Right => {
                if self.x + SPEED >= width - 1 {
                    self.x_direction = self.x_direction.opposite();
                }
            },
            Movement::Left => {
                if self.x <= 1 {
                    self.x_direction = self.x_direction.opposite();
                }
            },
            _ => {}
        }
    }

    pub fn detect_ball_paddle_collision(&mut self, paddle: &Paddle) {
        if self.y == paddle.y {
            if self.x == paddle.x || self.x == paddle.x + 4 {
                self.y_direction = self.y_direction.opposite();
                self.y_speed_factor = SLOW_SPEED_FACTOR;
                self.x_speed_factor = FAST_SPEED_FACTOR;
                self.y -= 2;
            } else if self.x == paddle.x + 1 || self.x == paddle.x + 3 {
                self.y_direction = self.y_direction.opposite();
                self.y_speed_factor = DEFAULT_SPEED_FACTOR;
                self.x_speed_factor = DEFAULT_SPEED_FACTOR;
                self.y -= 2;
            } else if self.x == paddle.x + 2 {
                self.y_direction = self.y_direction.opposite();
                self.y_speed_factor = FAST_SPEED_FACTOR;
                self.x_speed_factor = SPEED_ZERO;
                self.y -= 2;
            }
        }
    }


    pub fn reset(&mut self) {
        self.x = 15;
        self.y = 15;
        self.y_direction = Movement::Down;
    }
}
