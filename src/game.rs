use piston_window::{Context, G2d, Key};
use piston_window::types::Color;

use crate::drawing::{draw_rectangle, draw_block};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const PADDLE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];


#[derive(Clone, Copy, PartialEq)]
pub enum Movement {
    Left, Right, Iddle
}


pub struct Game {
    width: i32,
    height: i32,
    player_1: Paddle,
    player_2: Paddle,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            width: width,
            height: height,
            player_1: Paddle::new((width / 2) - 2, height - 3),
            player_2: Paddle::new((width / 2) - 2, 2),
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.player_1.draw(con, g);
        self.player_2.draw(con, g);

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    pub fn update(&mut self) {
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.player_1.move_paddle(Movement::Left),
            Key::Right => self.player_1.move_paddle(Movement::Right),
            Key::A => self.player_2.move_paddle(Movement::Left),
            Key::D => self.player_2.move_paddle(Movement::Right),
            _ => {}
        }
        // let dir_player_1 = match key {
        //     Key::Left => Some(Movement::Left),
        //     Key::Right => Some(Movement::Right),
        //     _ => None
        // };
        // let dir_player_2 = match key {
        //     Key::A => Some(Movement::Left),
        //     Key::D => Some(Movement::Right),
        //     _ => None
        // };
        // self.player_1.move_paddle(dir_player_1);
        // self.player_2.move_paddle(dir_player_2);
    }
}

pub struct Paddle {
    x: i32,
    y: i32,
    moving_direction: Movement,
}

impl Paddle {
    pub fn new(x: i32, y: i32) -> Paddle {
        Paddle {
            x: x,
            y: y,
            moving_direction: Movement::Iddle
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Paddle: 4 blocks long
        draw_block(PADDLE_COLOR, self.x, self.y, con, g);
        draw_block(PADDLE_COLOR, self.x + 1, self.y, con, g);
        draw_block(PADDLE_COLOR, self.x + 2, self.y, con, g);
        draw_block(PADDLE_COLOR, self.x + 3, self.y, con, g);
    }

    pub fn move_paddle(&mut self, dir: Movement) {
        match dir {
            Movement::Left => self.x -= 1,
            Movement::Right => self.x += 1,
            Movement::Iddle => {}
        }
        // match dir {
        //     Some(d) => self.moving_direction = d,
        //     None => {}
        // }
        // match self.moving_direction {
        //     Movement::Left => self.x -= 1,
        //     Movement::Right => self.x += 1,
        //     Movement::Iddle => {}
        // }
    }
}
