extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30u32, 30u32);
    let (window_width, window_height) = to_coord_u32(width, height);

    let mut window: PistonWindow = WindowSettings::new("Snake", [window_width, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
