extern crate itertools_num;

use controls::Direction;
use itertools_num::linspace;
use piston_window::types::Color;
use piston_window::*;
use piston_window::{rectangle, Context, G2d};
use std::fmt;
// use std::thread;
// use std::time;

pub const BLOCK_SIZE: f64 = 5.0;
pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 0.5];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const HORIZONTAL_GRID_NUMBER: usize = 20;
pub const VERTICAL_GRID_NUMBER: usize = 20;
pub const WIDTH: i32 = 150;
pub const HEIGHT: i32 = 100;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Linea {
    pub start: Point,
    pub end: Point,
    pub thickness: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Linea {
    pub fn new(start: Point, end: Point, thickness: f64) -> Self {
        Linea {
            start,
            end,
            thickness,
        }
    }

    pub fn vertical_grid(&self, grid_number: usize) -> Vec<Linea> {
        let width = self.end.x - self.start.x; // * BLOCK_SIZE;
        let height = self.end.y - self.start.y; // * BLOCK_SIZE;
        let mut start = linspace(self.start.x, width - self.thickness, grid_number)
            .map(|x| Point::new(x, self.start.y));
        let mut end = linspace(self.start.x, width - self.thickness, grid_number)
            .map(|x| Point::new(x, height));
        let mut vertical_lines = Vec::new();
        for _ in 0..grid_number {
            vertical_lines.push(Linea::new(
                start.next().unwrap(),
                end.next().unwrap(),
                self.thickness,
            ));
        }
        vertical_lines
    }

    pub fn horizontal_grid(&self, grid_number: usize) -> Vec<Linea> {
        let width = self.end.x - self.start.x; // * BLOCK_SIZE;
        let height = self.end.y - self.start.y; // * BLOCK_SIZE;
        let mut start =
            linspace(0.0, height - self.thickness, grid_number).map(|y| Point::new(0.0, y));
        let mut end =
            linspace(0.0, height - self.thickness, grid_number).map(|y| Point::new(width, y));
        let mut horizontal_lines = Vec::new();
        for _ in 0..grid_number {
            horizontal_lines.push(Linea::new(
                start.next().unwrap(),
                end.next().unwrap(),
                self.thickness,
            ));
        }
        horizontal_lines
    }

    pub fn create_grid() -> (Vec<Self>, Vec<Self>) {
        let left_edge = Point::new(0.0, 0.0);
        let bottom_right_edge = Point::new(to_coord(WIDTH), to_coord(HEIGHT));
        // Setting up the grid lines
        let vertical_line = Self::new(left_edge, bottom_right_edge, 1.0);
        let horizontal_line = Self::new(left_edge, bottom_right_edge, 1.0);
        let vertical_lines = Self::vertical_grid(&vertical_line, VERTICAL_GRID_NUMBER + 2);
        let horizontal_lines = Self::horizontal_grid(&horizontal_line, HORIZONTAL_GRID_NUMBER + 2);

        (vertical_lines, horizontal_lines)
    }

    pub fn cell_size(&self, grid_number: usize) {
        let width = self.end.x - self.start.x;
        let height = self.end.y - self.start.y;
        let cell_width = linspace(0.0, width, grid_number)
            .nth(2)
            .unwrap();
        let cell_height = linspace(0.0, height, grid_number)
            .nth(2)
            .unwrap();
    }
}

pub struct Square {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}, {}", self.x, self.y)
    }
}

impl Square {
    pub fn initial_position() -> Square {
        Square { x: 0, y: 0 }
    }

    pub fn curr_position(&self) -> Square {
        Self {
            x: self.x,
            y: self.y,
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

    pub fn update_square(&mut self, dir: Option<Direction>) {
        // let mut position = ;
        if let Some(mut d) = dir { Direction::move_square(&mut d, self)};
    }
}

pub fn to_coord(game_coord: i32) -> f64 {
    f64::from(game_coord) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(color: Color, column: i32, row: i32, con: &Context, g: &mut G2d) {
    let (grid_width, grid_height) = grid_settings();
    // let square = rectangle::square(1.0, 1.0, grid_height);
    let rect = rectangle::rectangle_by_corners(1.0, 1.0, grid_width - 1.0, grid_height - 1.0);
    let square_position = (grid_width * f64::from(column), grid_height * f64::from(row));

    rectangle(
        color,
        rect,
        con.transform.trans(square_position.0, square_position.1),
        g,
    );
}

pub fn grid_settings() -> (f64, f64) {
    let grid_width = linspace(0.0, to_coord(WIDTH), VERTICAL_GRID_NUMBER + 2)
        .nth(2)
        .unwrap();
    let grid_height = linspace(0.0, to_coord(HEIGHT), HORIZONTAL_GRID_NUMBER + 2)
        .nth(2)
        .unwrap();
    (grid_width, grid_height)
}

//            ellipse(green, circle, center, g);
//            let proc_num = num_cpus::get();
//            thread::spawn(move ||
//                for linea in &vertical_lines {
//            let (tx, rx) = mpsc::channel();
//            vertical_lines.iter().for_each(|linea| {tx.send(line(white, 1.0, [linea.start.x, linea.start.y, linea.end.x, linea.end.y], c.transform, g)).unwrap();});
//            drop(tx);
//            let (tx, rx) = mpsc::channel();
//            horizontal_lines.iter().for_each(|linea| {tx.send(line(white, 1.0, [linea.start.x, linea.start.y, linea.end.x, linea.end.y], c.transform, g)).unwrap();});
//            drop(tx);
//                });
//            handle.join().unwrap();

//            horizontal_lines.iter().for_each(|linea| line(white, 1.0, [linea.start.x, linea.start.y, linea.end.x, linea.end.y], c.transform, g));

//            line(white, 1.0, [50.0, 0.0, 50.0, 740.0], c.transform, g);
