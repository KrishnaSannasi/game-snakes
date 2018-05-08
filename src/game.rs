use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rect};

const FOOD_COLOR: Color = [0.8, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const FOREGROUND_COLOR: Color = [1.00, 1.00, 1.00, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food: Option<(i32, i32)>,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food: Some((6, 4)),
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            if d == self.snake.head_dir().opp() {
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if let None = self.food {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if let Some(food) = self.food {
            draw_block(FOOD_COLOR, food.0, food.1, con, g);
        }

        draw_rect(BORDER_COLOR, 0, 0, self.width, self.height, con, g);
        draw_rect(FOREGROUND_COLOR, 1, 1, self.width - 2, self.height - 2, con, g);

        if self.game_over {
            draw_rect(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    fn check_eating(&mut self) {
        let (x, y) = self.snake.head_position();

        if let Some(food) = self.food {
            if food.0 == x && food.1 == y {
                self.food = None;
                self.snake.restore_tail();
            }
        }
    }

    fn check_alive(&self, dir: Option<Direction>) -> bool {
        let (x, y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(x, y) {
            return false;
        }

        x > 0 && y > 0 && x < self.width - 1 && y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut x = rng.gen_range(1, self.width - 1);
        let mut y = rng.gen_range(1, self.height - 1);

        while self.snake.overlap_tail(x, y) {
            x = rng.gen_range(1, self.width - 1);
            y = rng.gen_range(1, self.height - 1);
        }

        self.food = Some((x, y));
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_alive(dir) {
            self.snake.move_dir(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food = Some((6, 4));
        self.game_over = false;
    }
}
