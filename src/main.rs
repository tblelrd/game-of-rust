use std::env;
use raylib::prelude::*;

mod cell;
mod grid;

use grid::Grid;

const BACKGROUND_COLOR: Color = Color::BLACK;
const CELL_COLOR: Color = Color::WHITE;
const CELL_OUTLINE: Color = Color::BLACK;

const TARGET_FPS: u32 = 120;
const UPDATE_FRAMES: u32 = TARGET_FPS / 120; // How many frames per tick 
const WAIT_TIME: u32 = TARGET_FPS / 2; // How many frames after mouse down to start updating again.
const TIME_TO_RED: u32 = UPDATE_FRAMES * 2; // How many frames it takes to become red.

const MAX_THREADS: usize = 20;

fn invalid_params () {
    println!("Expected parameters: ./conway <width> <height> <width of cell>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return invalid_params();
    }

    let width: i32 = match args[1].parse() {
        Ok(width) => width,
        Err(_) => return invalid_params(),
    };

    let height: i32 = match args[2].parse() {
        Ok(height) => height,
        Err(_) => return invalid_params(),
    };

    let cell_width: i32 = match args[3].parse() {
        Ok(cell_size) => cell_size,
        Err(_) => return invalid_params(),
    };

    let window_width = width * cell_width;
    let window_height = height * cell_width;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Hello, world!")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut grid = Grid::new(width, height, cell_width);
    let mut last_state: Option<bool> = None;
    let mut frames = 0;
    let mut paused_frames = 0;
    while !rl.window_should_close() {

        let left_clicked = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let right_clicked = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT);

        if paused_frames <= 0{
            frames += 1;
            if frames % UPDATE_FRAMES == 0 {
                // let now = std::time::Instant::now();
                grid.step();
                // println!("{:.2?}", now.elapsed());
            }
        } else {
            paused_frames -= 1;
        }

        // if right_clicked {
        //     paused_frames = WAIT_TIME;
        //     frames = 0;
        // }
        //
        // Do mouse stuff
        if left_clicked {
            // Reset frames
            paused_frames = WAIT_TIME;
            frames = 0;

            let mouse_position = rl.get_mouse_position();
            let state = match last_state {
                Some(state) => state,
                None => if let Some(cell) = grid.get_cell_at(&mouse_position) {
                    !cell.state
                } else {
                    true
                },
            };

            grid.set_cell_at(&mouse_position, state);
            last_state = Some(state);
        } else {
            last_state = None;
        }

        // Do draw stuff
        let frame_time = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        // dbg!(1.0 / frame_time);

        d.clear_background(BACKGROUND_COLOR);
        if left_clicked || right_clicked { grid.draw(&mut d); }
        // grid.draw(&mut d);

        d.draw_text(&format!("FPS: {}", 1.0 / frame_time), 0, 0, 24, Color::WHITE);
    }
}
