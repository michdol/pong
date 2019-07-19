use piston_window::{Context, G2d, Key};
use piston_window::types::Color;

use crate::ball::{Ball};
use crate::drawing::{draw_rectangle};
use crate::paddle::{Paddle};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
pub const SPEED: i32 = 1;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Movement {
    Left, Right, Up, Down
}

impl Movement {
    pub fn opposite(&self) -> Movement {
        return match self {
            Movement::Right => Movement::Left,
            Movement::Left => Movement::Right,
            Movement::Up => Movement::Down,
            Movement::Down => Movement::Up
        };
    }
}


pub struct Game {
    width: i32,
    height: i32,
    ball: Ball,
    player_1: Paddle,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width: width,
            height: height,
            ball: Ball::new(width / 2 + 1, height - 14),
            player_1: Paddle::new((width / 2) - 2, height - 2),
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.ball.draw(con, g);
        self.player_1.draw(con, g);

        // draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        // draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    pub fn update(&mut self) {
        self.ball.update();
        self.player_1.detect_paddle_wall_collision(self.width);
        self.ball.detect_ball_wall_collision(self.width);
        self.ball.detect_ball_paddle_collision(&self.player_1);
        self.player_1.update();
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.player_1.set_direction(Some(Movement::Left)),
            Key::Right => self.player_1.set_direction(Some(Movement::Right)),
            Key::R => self.ball.reset(),
            _ => {}
        };
    }

    pub fn key_released(&mut self) {
        self.player_1.stop();
    }
}
