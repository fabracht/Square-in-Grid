#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate itertools_num;
//extern crate num_cpus;
extern crate piston_window;
//extern crate rayon;
//extern crate shapes;

mod controls;
mod draw;
mod mechanics;
// mod game;

use piston_window::*;
// use piston_window::types::Color;
// use itertools_num::linspace;
// use std::f64;
use std::thread;
//use std::sync::mpsc;
//use rayon::prelude::*;
// use std::fmt;
use std::time;

use controls::*;
use draw::*;
use mechanics::*;
// use game::Game;

fn main() {
    let ten_millis = time::Duration::from_millis(100);
    // Setting up the grid
    let (_grid_width, grid_height) = grid_settings();
    let (vertical_lines, horizontal_lines) = Linea::create_grid();

    // Creating the grid lines iterator
    // let mut column_iter = (0..VERTICAL_GRID_NUMBER - 1).cycle();
    // let mut row = 0;

    // Create the initial Canvas
    let mut window: PistonWindow =
        WindowSettings::new("Square Grid", [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut square = draw::Square::initial_position();

    // Main event loop
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            square.key_pressed(key);
            println!("{}", square.curr_position());
        };
        window.draw_2d(&event, |c, g| {
            // Clear the window
            clear(BACK_COLOR, g);
            //Forms and colors
            // let _square = rectangle::square(1.0, 1.0, grid_height);
            // let _center = c.transform.trans(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);

            //Draw the vertical lines one by one
            vertical_lines.iter().for_each(|linea| {
                line(
                    WHITE,
                    1.0,
                    [linea.start.x, linea.start.y, linea.end.x, linea.end.y],
                    c.transform,
                    g,
                )
            });
            //Draw the horizontal lines one by one
            horizontal_lines.iter().for_each(|linea| {
                line(
                    WHITE,
                    1.0,
                    [linea.start.x, linea.start.y, linea.end.x, linea.end.y],
                    c.transform,
                    g,
                )
            });

            // let column = column_iter.next().unwrap();
            // if let Some(Button::Keyboard(key)) = event.press_args() {
            //     Game::key_pressed(key);
            // }
            // let column = column_iter.next().unwrap();
            /* let (mut column, mut row) = (
                game.square.x % WIDTH as usize,
                game.square.y % WIDTH as usize,
            ); */

            draw_rectangle(RED, square.x, square.y, &c, g);

            // thread::sleep(ten_millis);
        });
    }
}
