use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_cood(x: i32, y: i32) -> (f64, f64) {
    ((x as f64) * BLOCK_SIZE, (y as f64) * BLOCK_SIZE)
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let (gui_x, gui_y) = to_cood(x, y);
    
    rectangle(
        color, 
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g
    );
}

pub fn draw_rect(color: Color, x: i32, y: i32, width: i32, height: f64, con: &Context, g: &mut G2d) {
    let (gui_x, gui_y) = to_cood(x, y);
    
    rectangle(
        color, 
        [gui_x, gui_y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g
    );
}
