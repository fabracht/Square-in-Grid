use draw::*;
use piston_window::*;
use std::time;
use std::thread;
use mechanics;


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    Jump,
}

impl Movement {
    pub fn move_square(&mut self, square: &mut Square) {
        let last_position: Square = Square::curr_position(square);
        let hbarrier = VERTICAL_GRID_NUMBER as i32;
        let vbarrier = HORIZONTAL_GRID_NUMBER as i32;
        match *self {
            Movement::Up => {
                square.x = last_position.x;
                square.y = match last_position.y {
                    0 => vbarrier,
                    _ => last_position.y - 1,
                };
            }
            Movement::Down => {
                square.x = last_position.x;
                square.y = match last_position.y {
                    a if a == vbarrier => 0,
                    _ => last_position.y + 1,
                };
            }
            Movement::Left => {
                square.x = match last_position.x {
                    0 => hbarrier,
                    _ => last_position.x - 1,
                };
                square.y = last_position.y;
            }
            Movement::Right => {
                square.x = match last_position.x {
                    a if a == hbarrier => 0,
                    _ => last_position.x + 1,
                };
                square.y = last_position.y;
            }
            Movement::Jump => {
                square.x = last_position.x;
                square.vel = 3.0;
                if square.vel >= 0.0 {
                    mechanics::jump_mechanics(square.vel);
                    square.y = last_position.y - 1;
                } else if square.vel >= -3.0 {
                    square.y = last_position.y + 1;
                }
                //thread::sleep(ten_millis);

            }

        }
    }
}