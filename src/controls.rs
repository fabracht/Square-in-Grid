use draw::*;
use piston_window::*;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn move_square(&mut self, square: &mut Square) {
        let last_position: Square = Square::curr_position(square);
        let hbarrier = VERTICAL_GRID_NUMBER as i32;
        let vbarrier = HORIZONTAL_GRID_NUMBER as i32;
        match *self {
            Direction::Up => {
                square.x = last_position.x;
                square.y = match last_position.y {
                    0 => vbarrier,
                    _ => last_position.y - 1,
                };
            }
            Direction::Down => {
                square.x = last_position.x;
                square.y = match last_position.y {
                    a if a == vbarrier => 0,
                    _ => last_position.y + 1,
                };
            }
            Direction::Left => {
                square.x = match last_position.x {
                    0 => hbarrier,
                    _ => last_position.x - 1,
                };
                square.y = last_position.y;
            }
            Direction::Right => {
                square.x = match last_position.x {
                    a if a == hbarrier => 0,
                    _ => last_position.x + 1,
                };
                square.y = last_position.y;
            } // Direction::Down => Square {
        }
    }
}
