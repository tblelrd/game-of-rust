use std::thread;

use raylib::prelude::*;
use crate::cell::{check_cell_conway, check_cell_test, Cell};

pub struct Grid {
    width: i32,
    height: i32,
    cell_width: i32,
    cells: Vec<Cell>,
}

trait WakeUp {
    fn wake_up_at(&mut self, index: usize);
}

impl WakeUp for Vec<Cell> {
    fn wake_up_at(&mut self, index: usize) {
        self[index].should_update = true;
    }
}

fn get_index_from_position(position: (i32, i32), size: (i32, i32)) -> usize {
    let (row, col) = position;
    let (width, height) = size;

    let mut new_col = col % width;
    if col < 0 { new_col += width; }

    let mut new_row = row % height;
    if row < 0 { new_row += height; }

    (new_col + (new_row * width)) as usize
}


impl Grid {
    pub fn new(width: i32, height: i32, cell_width: i32) -> Grid {
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

    pub fn step(&mut self) {
        // let mut new_cells: Vec<Cell> = self.cells.clone();

        let mut handles = Vec::with_capacity(self.height as usize);
        for row in 0..self.height {
            let width = self.width;
            let height = self.height;

            handles.push(thread::spawn(move || {
                let row_cells: Vec<Cell> = Vec::with_capacity(width as usize);

                for col in 0..width {
                    let current_index = get_index_from_position((row, col), (width, height));
                }
            }));
            // for col in 0..self.width {
            //     let current_index = self.get_index_from_position((row, col));
            //     let new_cell = &mut new_cells[current_index];
            //
            //     if new_cell.state { new_cell.frames_alive += 1; }
            //     if !new_cell.should_update { continue; }
            //
            //
            //     let neighbors = self.get_neighbors(&self.cells, (row, col));
            //     new_cell.set_state(check_cell_test(self.cells[current_index].state, neighbors));
            // }
        }

        // for i in 0..new_cells.len() {
        //     if !new_cells[i].should_update { continue; }
        //     let position = self.get_position_from_index(i);
        //     if 
        //     (new_cells[i].state == self.cells[i].state)
        //     &&
        //     (self.get_neighbors(&self.cells, position) == self.get_neighbors(&new_cells, position))
        //     {
        //         new_cells[i].should_update = false;
        //     } else {
        //         self.wake_up_neighbors(&mut new_cells, position);
        //     }
        // }

        self.cells = vec!();
    }

    pub fn get_neighbors(&self, cells: &Vec<Cell>, position: (i32, i32)) -> u8 {
        let (row, col) = position;

        let tl = self.get_state_at(cells, (row - 1, col - 1));
        let tc = self.get_state_at(cells, (row - 1, col));
        let tr = self.get_state_at(cells, (row - 1, col + 1));
        let ml = self.get_state_at(cells, (row, col - 1));
        let mr = self.get_state_at(cells, (row, col + 1));
        let bl = self.get_state_at(cells, (row + 1, col - 1));
        let bc = self.get_state_at(cells, (row + 1, col));
        let br = self.get_state_at(cells, (row + 1, col + 1));

        let neighbors = tl as u8
            + tc as u8
            + tr as u8
            + ml as u8
            + mr as u8
            + bl as u8
            + bc as u8
            + br as u8;

        neighbors
    }

    pub fn wake_up_neighbors(&self, cells: &mut Vec<Cell>, position: (i32, i32)) {
        let (row, col) = position;

        cells.wake_up_at(self.get_index_from_position((row - 1, col - 1)));
        cells.wake_up_at(self.get_index_from_position((row - 1, col)));
        cells.wake_up_at(self.get_index_from_position((row - 1, col + 1)));
        cells.wake_up_at(self.get_index_from_position((row, col - 1)));
        cells.wake_up_at(self.get_index_from_position((row, col + 1)));
        cells.wake_up_at(self.get_index_from_position((row + 1, col - 1)));
        cells.wake_up_at(self.get_index_from_position((row + 1, col)));
        cells.wake_up_at(self.get_index_from_position((row + 1, col + 1)));
    }

    pub fn get_state_at(&self, cells: &Vec<Cell>, position: (i32, i32)) -> bool {
        let (row, col) = position;

        let mut new_col = col % self.width;
        if col < 0 { new_col += self.width; }

        let mut new_row = row % self.height;
        if row < 0 { new_row += self.height; }

        let index = self.get_index_from_position((new_row, new_col));
        cells[index].state
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for i in 0..self.cells.len() {
            let col = (i as i32) % self.width;
            let row = (i as i32) / self.width;

            let x = col * self.cell_width;
            let y = row * self.cell_width;


            let cell = &self.cells[i];
            if !cell.should_update && !cell.state { continue; }

            let color = cell.get_color();

            draw_handle.draw_rectangle(x, y, self.cell_width, self.cell_width, color);

            if self.cell_width <= 2 { continue; }
            draw_handle.draw_rectangle_lines(x, y, self.cell_width, self.cell_width, crate::CELL_OUTLINE);
        }
    }

    pub fn get_position_from_mouse(&self, mouse_position: &Vector2) -> (i32, i32) {
        let Vector2 {
            x,
            y,
        } = mouse_position;

        let col = (x.clone() as i32) / self.cell_width;
        let row = (y.clone() as i32) / self.cell_width;

        (row, col)
    }

    pub fn get_index_from_position(&self, position: (i32, i32)) -> usize {
        let (row, col) = position;

        let mut new_col = col % self.width;
        if col < 0 { new_col += self.width; }

        let mut new_row = row % self.height;
        if row < 0 { new_row += self.height; }

        (new_col + (new_row * self.width)) as usize
    }

    pub fn get_position_from_index(&self, index: usize) -> (i32, i32) {
        (
            index as i32 / self.width,
            index as i32 % self.width,
        )
    }

    pub fn get_cell_at_mut(&mut self, mouse_position: &Vector2) -> Option<&mut Cell> {
        let position = self.get_position_from_mouse(mouse_position);
        let index = self.get_index_from_position(position);
        self.cells.get_mut(index)
    }

    pub fn get_cell_at(&self, mouse_position: &Vector2) -> Option<&Cell> {
        let position = self.get_position_from_mouse(mouse_position);
        let index = self.get_index_from_position(position);
        self.cells.get(index)
    }

    pub fn set_cell_at(&mut self, mouse_position: &Vector2, state: bool) {
        let position = self.get_position_from_mouse(mouse_position);
        let mut temp_cells = self.cells.clone();
        self.wake_up_neighbors(&mut temp_cells, position);
        self.cells = temp_cells;

        match self.get_cell_at_mut(mouse_position) {
            Some(cell) => cell.set_state(state),
            None => { dbg!("Something went wrong when toggling cell"); },
        }
    }
}

