use piston_window::{Context, G2d, Key};
use piston_window::types::Color;

use crate::drawing::{draw_rectangle, draw_block};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];


pub struct Game {
    width: i32,
    height: i32
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width: width,
            height: height
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
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
