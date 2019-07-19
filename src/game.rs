use piston_window::{Context, G2d, Key};
use piston_window::types::Color;

use crate::drawing::{draw_rectangle, draw_block};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const PADDLE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];
const BALL_COLOR: Color = [0.984, 0.537, 0.56, 1.0];
const SPEED: i32 = 1;
const PADDLE_LENGTH: i32 = 5;


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Movement {
    Left, Right, Up, Down
}

impl Movement {
    pub fn opposite(&self) -> Movement {
        let dir = match self {
            Movement::Right => Movement::Left,
            _ => Movement::Right
        };
        dir
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
            ball: Ball::new(5, 0),
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
        self.detect_paddle_wall_collision();
        self.detect_ball_wall_collision();
        self.player_1.update();
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Left => Some(Movement::Left),
            Key::Right => Some(Movement::Right),
            _ => None
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
            _ => {}
        }
    }

    fn detect_ball_wall_collision(&mut self) {
        match self.ball.x_direction {
            Movement::Right => {
                if self.ball.x + SPEED >= self.width - 1 {
                    self.ball.change_x_direction();
                }
            },
            Movement::Left => {
                if self.ball.x <= 1 {
                    self.ball.change_x_direction();
                }
            },
            _ => {}
        }
    }
}


pub struct Paddle {
    pub x: i32,
    y: i32,
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
}

const DEFAULT_SPEED_FACTOR: i32 = 15;

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
            x_speed_factor: DEFAULT_SPEED_FACTOR,
            y_speed_factor: DEFAULT_SPEED_FACTOR,
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

    pub fn change_x_direction(&mut self) {
        self.x_direction = self.x_direction.opposite();
        // match self.x_direction {
        //     Movement::Right => self.x_direction = Movement::Left,
        //     Movement::Left => self.x_direction = Movement::Right,
        //     _ => {}
        // }
    }
}
