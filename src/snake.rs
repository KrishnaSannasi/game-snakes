use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const INIT_SNAKE_LEN: u32 = 3;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn opp(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
struct Block {
    x: i32,
    y: i32
}

impl Block {
    fn new(x: i32, y: i32) -> Block {
        Block { x, y }
    }
}

pub struct Snake {
    dir: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body = LinkedList::new();

        for i in 1..INIT_SNAKE_LEN + 1 {
            body.push_back(Block::new(x + i as i32, y));
        }

        Snake {
            body, dir: Direction::Right,
            tail: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn move_dir(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.dir = d,
            None => ()
        };

        let (x, y) = self.head_position();
        let new_block = match self.dir {
            Direction::Up => Block {
                x, y: y - 1
            },
            Direction::Down => Block {
                x, y: y + 1
            },
            Direction::Left => Block {
                x: x - 1, y
            },
            Direction::Right => Block {
                x: x + 1, y
            }
        };

        self.body.push_back(new_block);
        self.tail = self.body.pop_back();
    }

    pub fn head_dir(&self) -> Direction {
        self.dir
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (x, y) = self.head_position();

        let move_dir = match dir {
            Some(d) => d,
            None => self.dir
        };

        match move_dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y)
        }
    }

    pub fn restore_tail(&mut self) {
        match self.tail {
            Some(tail) => self.body.push_back(tail.clone()),
            None => ()
        };
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut pos = 1;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }


            pos += 1;
            if pos == self.body.len() {
                break;
            }
        }

        return false;
    }
}