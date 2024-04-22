mod timer;
mod utils;

use fixedbitset::FixedBitSet;
use rand::Rng;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn as_flag(&self) -> bool {
        match self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }

    fn from_flag(flag: bool) -> Cell {
        match flag {
            false => Cell::Dead,
            true => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    buffer0: FixedBitSet,
    buffer1: FixedBitSet,
    active: u8,
}

#[wasm_bindgen]
impl Universe {
    pub fn index(width: u32, row: u32, column: u32) -> usize {
        (row * width + column) as usize
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        let cells = self.active_buffer_mut();
        log!("Toggling cell at index: {}", idx);
        cells.toggle(idx);
    }

    fn active_buffer(&self) -> &FixedBitSet {
        match self.active {
            0 => &self.buffer0,
            1 => &self.buffer1,
            _ => panic!("Invalid active buffer"),
        }
    }

    fn active_buffer_mut(&mut self) -> &mut FixedBitSet {
        match self.active {
            0 => &mut self.buffer0,
            1 => &mut self.buffer1,
            _ => panic!("Invalid active buffer"),
        }
    }

    fn buffers(&mut self) -> (&FixedBitSet, &mut FixedBitSet) {
        match self.active {
            0 => (&self.buffer0, &mut self.buffer1),
            1 => (&self.buffer1, &mut self.buffer0),
            _ => panic!("Invalid active buffer"),
        }
    }

    fn toggle_active(&mut self) {
        self.active = 1 - self.active;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        Universe::index(self.width, row, column)
    }

    fn live_neighbour_count(
        cells: &FixedBitSet,
        height: u32,
        width: u32,
        row: u32,
        column: u32,
    ) -> u8 {
        let mut count = 0;
        for delta_row in [height - 1, 0, 1].iter().cloned() {
            for delta_col in [width - 1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % height;
                let neighbour_col = (column + delta_col) % width;
                let idx = Universe::index(width, neighbour_row, neighbour_col);
                count += cells[idx] as u8;
            }
        }
        count
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns an array view of the cells in the universe.
    pub fn cells(&self) -> js_sys::Uint8Array {
        let cells = self.active_buffer();
        let u8_cells = cells.as_slice().as_ptr() as *const u8;
        unsafe {
            // this is a reinterpret cast from *const u8 to slice<u8>
            let slice = std::slice::from_raw_parts(u8_cells, cells.len() / 8);
            js_sys::Uint8Array::view(slice)
        }
    }

    /// Invokes tick n times.
    pub fn tick_n(&mut self, times: u32) {
        // let _timer = timer::Timer::new("Universe::tick_n");
        for _ in 0..times {
            self.tick();
        }
    }

    /// Move forward the universe by one tick using the conway game of life rules.
    pub fn tick(&mut self) {
        let height = self.height;
        let width = self.width;
        let (cells, next) = self.buffers();
        for row in 0..height {
            for col in 0..width {
                let idx = Universe::index(width, row, col);
                let cell = Cell::from_flag(cells[idx]);
                let live_neighbours =
                    Universe::live_neighbour_count(cells, height, width, row, col);
                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next.set(idx, next_cell.as_flag());
            }
        }
        self.toggle_active();
    }

    /// Randomizes the universe.
    pub fn randomize(&mut self) {
        let _cells = Universe::build_universe_random(self.width, self.height);
        *self = Universe::new_with_cells(self.width, self.height, _cells.as_slice());
    }

    /// Resets the universe to an empty universe.
    pub fn reset(&mut self) {
        *self = Universe::new_with_cells(self.width, self.height, &[]);
    }

    fn build_universe_default(width: u32, height: u32) -> Vec<usize> {
        (0..(width * height) as usize)
            .filter(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    true
                } else {
                    false
                }
            })
            .collect()
    }

    fn build_universe_random(width: u32, height: u32) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        (0..(width * height) as usize)
            .filter(|_| {
                if rng.gen_range(0.0..1.0) < 0.5 {
                    true
                } else {
                    false
                }
            })
            .collect()
    }

    fn build_universe_spaceship(width: u32, height: u32) -> [usize; 5] {
        let center = (width / 2, height / 2);
        // x and y are swapped because x is the column and y is the row.
        let index = |x, y| Universe::index(width, y, x);
        let spaceship_width = 1;
        let spaceship_height = 1;
        [
            // *
            index(center.0, center.1),
            // **
            index(center.0 + spaceship_width, center.1),
            //  *
            //   **
            index(center.0 - spaceship_width, center.1 + spaceship_height),
            //  **
            //   **
            index(center.0, center.1 + spaceship_height),
            //  **
            //   **
            //   *
            // index(center.0, center.1 - spaceship_height),
            //  **
            //   **
            //  **
            index(center.0 - spaceship_width, center.1 - spaceship_height),
        ]
    }

    /// Create a new Universe.
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();
        let _cells0 = Universe::build_universe_default(width, height);
        let _cells1 = Universe::build_universe_spaceship(width, height);
        let _cells2 = Universe::build_universe_random(width, height);
        Universe::new_with_cells(width, height, _cells1.as_slice())
    }

    pub fn new_random(width: u32, height: u32) -> Universe {
        let _cells = Universe::build_universe_random(width, height);
        Universe::new_with_cells(width, height, _cells.as_slice())
    }

    pub fn new_with_cells(width: u32, height: u32, cells_to_set: &[usize]) -> Universe {
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);
        for index in cells_to_set.iter().cloned() {
            cells.set(index, true);
        }
        Universe {
            width,
            height,
            buffer0: cells,
            buffer1: FixedBitSet::with_capacity((width * height) as usize),
            active: 0,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cells = &self.active_buffer();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if cells[idx] { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
        }
        Ok(())
    }
}
