use piston_window::{Context, G2d, Key};
use piston_window::types::Color;

use crate::drawing::{draw_rectangle, draw_block};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const PADDLE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];
const SPEED: i32 = 1;
const PADDLE_LENGTH: i32 = 4;


#[derive(Clone, Copy, PartialEq)]
pub enum Movement {
    Left, Right, Iddle
}


pub struct Game {
    width: i32,
    height: i32,
    player_1: Paddle,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width: width,
            height: height,
            player_1: Paddle::new((width / 2) - 2, height - 3),
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.player_1.draw(con, g);

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    pub fn update(&mut self) {
        self.detect_paddle_wall_collision();
        self.player_1.update();
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Left => Movement::Left,
            Key::Right => Movement::Right,
            _ => Movement::Iddle
        };
        self.player_1.set_direction(dir);
    }

    pub fn key_released(&mut self, key: Key) {
        self.player_1.stop();
    }

    fn detect_paddle_wall_collision(&mut self) {
        match self.player_1.movement_direction {
            Movement::Right => {
                if self.player_1.x + SPEED >= self.width - PADDLE_LENGTH {
                    self.player_1.stop();
                }
            },
            Movement::Left => {
                if self.player_1.x - SPEED <= 0 {
                    self.player_1.stop();
                }
            },
            Movement::Iddle => {}
        }
    }
}

pub struct Paddle {
    pub x: i32,
    y: i32,
    pub movement_direction: Movement,
    length: i32
}

impl Paddle {
    pub fn new(x: i32, y: i32) -> Paddle {
        Paddle {
            x: x,
            y: y,
            movement_direction: Movement::Iddle,
            length: PADDLE_LENGTH,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Paddle: 4 blocks long
        for i in 0..self.length {
            draw_block(PADDLE_COLOR, self.x + i, self.y, con, g);
        }
    }

    pub fn update(&mut self) {
        match self.movement_direction {
            Movement::Left => {
                self.x -= SPEED;
            },
            Movement::Right => {
                self.x += SPEED;
            },
            Movement::Iddle => {}
        }
    }

    pub fn set_direction(&mut self, dir: Movement) {
        self.movement_direction = dir;
    }

    pub fn stop(&mut self) {
        self.movement_direction = Movement::Iddle;
    }
}
