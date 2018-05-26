use piston_window::types::Color;
use piston_window::*;

use controls::Direction;
use draw::*;

pub struct Game {
    pub square: Square,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            square: Square::initial_position(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        self.update_square(dir);
    }

    pub fn update_square(&mut self, dir: Option<Direction>) -> Self {
        let position = Square::curr_position(&mut self.square);
        match dir {
            Some(mut d) => Direction::move_square(&mut d, position),
            None => position,
        }
    }
}
