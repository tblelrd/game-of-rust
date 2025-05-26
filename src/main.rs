use std::env;
use raylib::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;
const CELL_COLOR: Color = Color::WHITE;
const CELL_OUTLINE: Color = Color::BLACK;

const TARGET_FPS: u32 = 120;
const UPDATE_FRAMES: u32 = TARGET_FPS / 20; // How many frames per tick 
const WAIT_TIME: u32 = TARGET_FPS / 2; // How many frames after mouse down to start updating again.

const TIME_TO_RED: u32 = UPDATE_FRAMES * 2; // How many frames it takes to become red.

#[derive(Clone, Debug)]
struct Cell {
    frames_alive: u16,
    state: bool,
}

impl Cell {
    fn new(state: bool) -> Cell {
        Cell {
            frames_alive: 0,
            state,
        }
    }

    fn set_state(&mut self, state: bool) {
        if !state { self.frames_alive = 0 }
        self.state = state;
    }

    fn get_color(&self) -> Color {
        if self.state {
            let t = (self.frames_alive as f32 / TIME_TO_RED as f32).clamp(0., 1.);

            CELL_COLOR.lerp(Color::RED, t)
        } else {
            BACKGROUND_COLOR
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(false)
    }
}

struct Grid {
    width: i32,
    height: i32,
    cell_width: i32,
    cells: Vec<Cell>,
}

// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
fn check_cell(state: bool, neighbors: u8) -> bool {
    if state {
        if neighbors < 2 {
            false
        } else if neighbors <= 3{
            true
        } else {
            false
        }
    } else {
        if neighbors == 3 {
            true
        } else {
            false
        }
    }
}

impl Grid {
    fn new(width: i32, height: i32, cell_width: i32) -> Grid {
        let size = width * height;

        let mut cells: Vec<Cell> = Vec::with_capacity(size.try_into().unwrap());
        for _ in 0..size {
            cells.push(Cell::default());
        }

        Grid {
            cells,
            width,
            height,
            cell_width,
        }
    }

    fn step(&mut self) {
        let mut new_cells: Vec<Cell> = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let tl = self.get_state_at((row - 1, col - 1));
                let tc = self.get_state_at((row - 1, col));
                let tr = self.get_state_at((row - 1, col + 1));
                let ml = self.get_state_at((row, col - 1));
                let mr = self.get_state_at((row, col + 1));
                let bl = self.get_state_at((row + 1, col - 1));
                let bc = self.get_state_at((row + 1, col));
                let br = self.get_state_at((row + 1, col + 1));

                let neighbors = tl as u8
                    + tc as u8
                    + tr as u8
                    + ml as u8
                    + mr as u8
                    + bl as u8
                    + bc as u8
                    + br as u8;

                let current_index = self.get_index_from_position((row, col));
                let cell = &mut new_cells[current_index];
                cell.set_state(check_cell(self.cells[current_index].state, neighbors));
                cell.frames_alive += 1;
            }
        }

        self.cells = new_cells;
    }

    fn get_state_at(&self, position: (i32, i32)) -> bool {
        let (row, col) = position;

        let mut new_col = col % self.width;
        if col < 0 { new_col += self.width; }

        let mut new_row = row % self.height;
        if row < 0 { new_row += self.height; }

        let index = self.get_index_from_position((new_row, new_col));

        self.cells[index].state
    }

    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for i in 0..self.cells.len() {
            let col = (i as i32) % self.width;
            let row = (i as i32) / self.width;

            let x = col * self.cell_width;
            let y = row * self.cell_width;

            let color = self.cells[i].get_color();
            draw_handle.draw_rectangle(x, y, self.cell_width, self.cell_width, color);
            draw_handle.draw_rectangle_lines(x, y, self.cell_width, self.cell_width, CELL_OUTLINE);
        }
    }

    fn get_position_from_mouse(&self, mouse_position: &Vector2) -> (i32, i32) {
        let Vector2 {
            x,
            y,
        } = mouse_position;

        let col = (x.clone() as i32) / self.cell_width;
        let row = (y.clone() as i32) / self.cell_width;

        (row, col)
    }

    fn get_index_from_position(&self, position: (i32, i32)) -> usize {
        let (row, col) = position;

        (col + (row * self.width)) as usize
    }

    fn get_cell_at_mut(&mut self, mouse_position: &Vector2) -> Option<&mut Cell> {
        let position = self.get_position_from_mouse(mouse_position);
        let index = self.get_index_from_position(position);
        self.cells.get_mut(index)
    }

    fn get_cell_at(&self, mouse_position: &Vector2) -> Option<&Cell> {
        let position = self.get_position_from_mouse(mouse_position);
        let index = self.get_index_from_position(position);
        self.cells.get(index)
    }

    fn set_cell_at(&mut self, mouse_position: &Vector2, state: bool) {
        match self.get_cell_at_mut(mouse_position) {
            Some(cell) => cell.set_state(state),
            None => { dbg!("Something went wrong when toggling cell"); },
        }
    }
}

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

        let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

        if paused_frames <= 0{
            frames += 1;
            if frames % UPDATE_FRAMES == 0 {
                grid.step();
            }
        } else {
            paused_frames -= 1;
        }

        // Do mouse stuff
        if mouse_down {
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
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(BACKGROUND_COLOR);
        grid.draw(&mut d);
    }
}
