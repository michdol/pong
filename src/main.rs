extern crate piston_window;

use piston_window::*;

mod drawing;
mod game;
mod paddle;
mod ball;

use drawing::to_gui_coord_u32;
use game::Game;


fn main() {
    let (width, height) = (38, 32);

    let mut window: PistonWindow =
        WindowSettings::new("Rusty Pong",
        [to_gui_coord_u32(width), to_gui_coord_u32(height)])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(_key)) = event.press_args() {
            game.key_pressed(_key);
        }
        if let Some(Button::Keyboard(_key)) = event.release_args() {
            game.key_released();
        }

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            game.draw(&context, graphics);
        });
        event.update(|_arg| {
            game.update();
        });
    }
}
