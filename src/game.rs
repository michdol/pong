use piston_window::{Context, G2d, Key};
use piston_window::types::Color;

use crate::drawing::{draw_rectangle, draw_block};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const PADDLE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];


pub struct Game {
    width: i32,
    height: i32,
    paddle: Paddle
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width: width,
            height: height,
            paddle: Paddle::new((width / 2) - 1, height - 2)
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.paddle.draw(con, g);

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    pub fn update(&mut self) {
        println!("Game::update");
    }

    pub fn key_pressed(&mut self, key: Key) {
        println!("Game::key_pressed");
    }
}

pub struct Paddle {
    x: i32,
    y: i32,
}

impl Paddle {
    pub fn new(x: i32, y: i32) -> Paddle {
        Paddle {
            x: x,
            y: y,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(PADDLE_COLOR, self.x, self.y, con, g);
        draw_block(PADDLE_COLOR, self.x + 1, self.y, con, g);
        draw_block(PADDLE_COLOR, self.x + 2, self.y, con, g);
        draw_block(PADDLE_COLOR, self.x + 3, self.y, con, g);
    }
}
