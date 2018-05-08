use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 20.0;

pub fn to_coord(x: u32, y: u32) -> (f64, f64) {
    ((x as f64) * BLOCK_SIZE, (y as f64) * BLOCK_SIZE)
}

pub fn to_coord_u32(x: u32, y: u32) -> (u32, u32) {
    (((x as f64) * BLOCK_SIZE) as u32, ((y as f64) * BLOCK_SIZE) as u32)
}

pub fn draw_block(color: Color, x: u32, y: u32, con: &Context, g: &mut G2d) {
    let (gui_x, gui_y) = to_coord(x, y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rect(
    color: Color,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    con: &Context,
    g: &mut G2d,
) {
    let (gui_x, gui_y) = to_coord(x, y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
