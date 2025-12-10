use std::{sync::{Arc, RwLock}, thread};

use raylib::prelude::*;
use crate::{cell::{check_cell_conway, check_cell_test, Cell}, MAX_THREADS};

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

pub fn get_position_from_index(index: usize, size: (i32, i32)) -> (i32, i32) {
    let (width, _height) = size;
    (
        index as i32 / width,
        index as i32 % width,
    )
}

pub fn get_neighbors(cells: &Vec<Cell>, position: (i32, i32), size: (i32, i32)) -> u8 {
    let (row, col) = position;

    let tl = get_state_at(cells, (row - 1, col - 1), size);
    let tc = get_state_at(cells, (row - 1, col), size);
    let tr = get_state_at(cells, (row - 1, col + 1), size);
    let ml = get_state_at(cells, (row, col - 1), size);
    let mr = get_state_at(cells, (row, col + 1), size);
    let bl = get_state_at(cells, (row + 1, col - 1), size);
    let bc = get_state_at(cells, (row + 1, col), size);
    let br = get_state_at(cells, (row + 1, col + 1), size);

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

pub fn wake_up_neighbors(cells: &mut Vec<Cell>, position: (i32, i32), size: (i32, i32)) {
    let (row, col) = position;

    cells.wake_up_at(get_index_from_position((row - 1, col - 1), size));
    cells.wake_up_at(get_index_from_position((row - 1, col), size));
    cells.wake_up_at(get_index_from_position((row - 1, col + 1), size));
    cells.wake_up_at(get_index_from_position((row, col - 1), size));
    cells.wake_up_at(get_index_from_position((row, col + 1), size));
    cells.wake_up_at(get_index_from_position((row + 1, col - 1), size));
    cells.wake_up_at(get_index_from_position((row + 1, col), size));
    cells.wake_up_at(get_index_from_position((row + 1, col + 1), size));
}

pub fn get_state_at(cells: &Vec<Cell>, position: (i32, i32), size: (i32, i32)) -> bool {
    let (row, col) = position;

    let index = get_index_from_position((row, col), size);
    cells[index].state
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
        let arc_cells = Arc::new(self.cells.clone());

        let mut handles = Vec::with_capacity(self.height as usize);
        // let now = std::time::Instant::now();
        for i in 0..MAX_THREADS {
            let width = self.width;
            let height = self.height;
            let cloned_cells = arc_cells.clone();

            let number_of_rows = height / MAX_THREADS as i32;

            handles.push(thread::spawn(move || {
                let mut row_cells: Vec<Cell> = Vec::with_capacity(width as usize);

                let start_index = i as i32 * number_of_rows;
                let end_index = if i == MAX_THREADS - 1{
                    height
                } else {
                    start_index + number_of_rows
                };

                for row in start_index..end_index {
                    for col in 0..width {
                        let current_index = get_index_from_position((row, col), (width, height));
                        let cell = cloned_cells[current_index].clone();
                        row_cells.push(cell);

                        let cell = row_cells.last_mut().unwrap();
                        cell.needs_redraw = false;
                        if cell.state {
                            cell.frames_alive += 1;
                            if cell.frames_alive < crate::TIME_TO_RED {
                                cell.needs_redraw = true;
                            }
                        }
                        if !cell.should_update { continue; }
                        
                        let neighbors = get_neighbors(&cloned_cells, (row, col), (width, height));
                        let previous_state = cell.state;
                        cell.set_state(check_cell_conway(cloned_cells[current_index].state, neighbors));
                        if previous_state != cell.state {
                            cell.needs_redraw = true;
                        }
                    }
                }

                row_cells
            }));
        }
        // println!("Step one - {:.2?}", now.elapsed());

        // let now = std::time::Instant::now();
        let mut new_cells: Vec<Cell> = Vec::with_capacity(self.cells.capacity());
        for handle in handles {
            let result = handle.join().unwrap();
            new_cells.extend(result);
        }

        // println!("Join one - {:.2?}", now.elapsed());

        let length = new_cells.len();
        let arc_new_cells = Arc::new(new_cells);

        // let now = std::time::Instant::now();
        let mut handles = Vec::with_capacity(length);
        for i in 0..MAX_THREADS {
            let size = (self.width, self.height);
            let cloned_new_cells = arc_new_cells.clone();
            let old_cells = arc_cells.clone();

            let number_of_rows = self.height / MAX_THREADS as i32;

            handles.push(thread::spawn(move || {
                let (width, height) = size;
                let mut column_changes = Vec::with_capacity(width as usize);

                let start_index = i as i32 * number_of_rows;
                let end_index = if i == MAX_THREADS - 1{
                    height
                } else {
                    start_index + number_of_rows
                };

                for row in start_index..end_index {
                    for col in 0..width {
                        let position = (row, col);
                        let i = get_index_from_position(position, size);

                        if !cloned_new_cells[i].should_update {
                            // column_changes.push(None)
                            continue;
                        }

                        if cloned_new_cells[i].state == old_cells[i].state &&
                        (get_neighbors(&old_cells, position, size) == get_neighbors(&cloned_new_cells, position, size))
                        {
                            column_changes.push((i, false))
                        } else {
                            column_changes.push((i, true))
                        }
                    }
                }

                column_changes
            }));
        }
        // println!("Step two - {:.2?}", now.elapsed());

        let mut changes = Vec::with_capacity(self.cells.capacity());
        // let now = std::time::Instant::now();
        for handle in handles {
            let column_changes = handle.join().unwrap();
            changes.extend(column_changes)
        }
        // println!("Join two - {:.2?}", now.elapsed());

        // let now = std::time::Instant::now();
        let mut final_cells = Arc::into_inner(arc_new_cells).unwrap();
        // let change_length = changes.len();
        for change in changes {
            let (i, should_update) = change;

            final_cells[i].should_update = should_update;
            let position = get_position_from_index(i, (self.width, self.height));
            if should_update {
                wake_up_neighbors(&mut final_cells, position, (self.width, self.height));
            }
        }
        // println!("Step three - {:.2?} with {} iterations", now.elapsed(), change_length);

        self.cells = final_cells;
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for i in 0..self.cells.len() {
            let cell = &self.cells[i];
            if !cell.should_update && !cell.state { continue; }
            // if !cell.needs_redraw { continue; }

            let col = (i as i32) % self.width;
            let row = (i as i32) / self.width;

            let x = col * self.cell_width;
            let y = row * self.cell_width;

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

    pub fn get_cell_at_mut(&mut self, mouse_position: &Vector2) -> Option<&mut Cell> {
        let position = self.get_position_from_mouse(mouse_position);
        let index = get_index_from_position(position, (self.width, self.height));
        self.cells.get_mut(index)
    }

    pub fn get_cell_at(&self, mouse_position: &Vector2) -> Option<&Cell> {
        let position = self.get_position_from_mouse(mouse_position);
        let index = get_index_from_position(position, (self.width, self.height));
        self.cells.get(index)
    }

    pub fn set_cell_at(&mut self, mouse_position: &Vector2, state: bool) {
        let position = self.get_position_from_mouse(mouse_position);
        let mut temp_cells = self.cells.clone();
        wake_up_neighbors(&mut temp_cells, position, (self.width, self.height));
        self.cells = temp_cells;

        match self.get_cell_at_mut(mouse_position) {
            Some(cell) => cell.set_state(state),
            None => { dbg!("Something went wrong when toggling cell"); },
        }
    }
}

