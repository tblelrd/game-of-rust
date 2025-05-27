use raylib::prelude::*;

#[derive(Clone, Debug)]
pub struct Cell {
    pub frames_alive: u16,
    pub state: bool,
    pub should_update: bool,
}

impl Cell {
    fn new(state: bool) -> Cell {
        Cell {
            frames_alive: 0,
            should_update: true,
            state,
        }
    }

    pub fn set_state(&mut self, state: bool) {
        if !state { self.frames_alive = 0 }
        self.should_update = true;
        self.state = state;
    }

    pub fn get_color(&self) -> Color {
        if self.state {
            let t = (self.frames_alive as f32 / crate::TIME_TO_RED as f32).clamp(0., 1.);

            crate::CELL_COLOR.lerp(Color::RED, t)
        } else {
            crate::BACKGROUND_COLOR
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(false)
    }
}

// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
pub fn check_cell(state: bool, neighbors: u8) -> bool {
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
