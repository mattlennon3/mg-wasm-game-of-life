mod utils;

use core::{fmt, panic};
use rand::Rng;

use wasm_bindgen::prelude::*;
use std::convert::TryInto;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        // let message = String::from("I like dogs");
        // unsafe {
        //     log!("{}", message);
        // }

        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_| {
                    Cell::Dead
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn all_dead(&mut self) -> () {
        let mut next = self.cells.clone();

        next = next.iter().map(|_cell| Cell::Dead).collect();

        self.cells = next;
    }

    pub fn randomise(&mut self) -> () {
        let mut next = self.cells.clone();

        let mut rng = rand::thread_rng();
        
        next = next.iter().map(|_cell| {
            let rand_bool = rng.gen::<bool>();
            match rand_bool {
                true => Cell::Alive,
                false => Cell::Dead
            }
        }).collect();

        self.cells = next;
    }


    // look at glider.png to see what we were following
    pub fn add_spaceship(&mut self, x: u32, y: u32) -> () {
        let mut next = self.cells.clone();

        let mut living_cells: Vec<usize> = Vec::new();

        // 1
        living_cells.push(self.get_index(x, y));
        // 2
        living_cells.push(self.get_index(x + 1, y));
        // 3
        living_cells.push(self.get_index(x + 2, y));
        // 4
        living_cells.push(self.get_index(x + 3, y));
        // 5
        living_cells.push(self.get_index(x, y + 1));
        // 6
        living_cells.push(self.get_index(x + 4, y + 1));
        // 7
        living_cells.push(self.get_index(x, y + 2));
        // 8
        living_cells.push(self.get_index(x + 1, y + 3));
        // 9
        living_cells.push(self.get_index(x + 4, y + 3));

        living_cells.iter().for_each(|cell| {
            next[*cell] = Cell::Alive;
        });

        self.cells = next;
    }

    pub fn place_item_on_grid(&mut self, x: u32, y: u32, input_string: &str) -> () {
        let mut next = self.cells.clone();
        let thing_to_place = self.parse_text_input(input_string);

        thing_to_place.iter().for_each(|(cell_x, cell_y)| {
            // let cell_x_u32: u32 = cell_x.try_into().expect("problem parsing");
            let index = self.get_index(cell_x + x, cell_y + y);
            next[index] = Cell::Alive;

        });

        self.cells = next;
    }

// Vec<(u32, u32)>
    fn parse_text_input(&self, input_string: &str) -> Vec<(usize, usize)> {
//         let example = String::from("..OOO...OOO

// O....O.O....O
// O....O.O....O
// O....O.O....O
// ..OOO...OOO

// ..OOO...OOO
// O....O.O....O
// O....O.O....O
// O....O.O....O

// ..OOO...OOO");

        // Splitting on "" produces an empty
        // string on either end of the array so we filter these about before collecting them as a Vector of &str.


        // Mini grid, stamp onto whole grid
        // Whole grid, with only this input on it, then merge them


        // Mini grid
        //   1 dimensional array
        //   Only contain coordinates

        let mut row = 0;
        let mut column = 0;

        let mut mini_grid: Vec<(usize, usize)> = Vec::new();

        // let example2 = String::from("OOO\nOOO\nOO..OO");

        input_string.split("")
            .filter(|x| !x.is_empty())
            .enumerate()
            .for_each(|(index, char, )| {
                unsafe {
                    match char {
                        // Living
                        "O" => {
                            mini_grid.push((row, column));
                            column += 1;
                            // log!("{}", char)
                        },
                        // Dead
                        "." => {
                            column += 1;
                        },
                        // Newline
                        "\n" => {
                            // We now know the width of the thing we've been passed
                            row = row + 1;
                            column = 0;
                        },
                        _ => log!("{}", char),
                    }
                }
        });
        
        unsafe {
            log!("{:?}", mini_grid);
        }

        return mini_grid;

    }

    
    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }


    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}