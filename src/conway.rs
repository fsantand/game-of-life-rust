use std::{
    cmp::{max, min},
    isize, usize,
};

use crate::get_random_color;

pub const GRID_SIZE: i8 = 80;
pub const CELL_SIZE: f64 = 16.0;
pub const TIME_BETWEEN_GENERATIONS: f64 = 0.1;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub alive: bool,
    pub color: Option<[f32; 4]>
}

impl Cell {
    pub fn new() ->Cell {
        Cell {
            alive: false,
            color: None
        }
    } 
}

#[derive(Debug)]
pub struct State {
    pub grid: Vec<Vec<Cell>>,
    pub running: bool,
    pub fps: f64,
    pub current_timer: f64,
    pub current_generation: u32,
}

impl State {
    pub fn new() -> State {
        State {
            grid: vec![vec![Cell::new(); GRID_SIZE as usize]; GRID_SIZE as usize],
            running: false,
            fps: TIME_BETWEEN_GENERATIONS,
            current_timer: TIME_BETWEEN_GENERATIONS,
            current_generation: 0,
        }
    }

    pub fn toggle_cell(&mut self, position: [usize; 2], is_alive: Option<bool>, color: Option<[f32;4]>) -> () {
        let mut cell = self.grid[position[0]][position[1]];
        match is_alive {
            Some(state) => cell.alive = state,
            None => cell.alive = !cell.alive,
        }

        match color {
            Some(c) => cell.color = Some(c),
            None => cell.color = None,
        }

        self.grid[position[0]][position[1]] = cell;
        println!(
            "Cell at {:?} changed to {:?}",
            position, cell
        );
    }

    pub fn toggle_simulation(&mut self) -> () {
        self.current_timer = self.fps;
        self.running = !self.running;
        println!("Toggle running: {:?}", self.running);
    }

    pub fn run_timer(&mut self, dt: f64) -> bool {
        if !self.running {
            return false;
        }
        self.current_timer -= dt;
        return self.current_timer <= 0.0;
    }

    pub fn generate_next_state(&mut self) {
        let mut cells_to_change: Vec<([usize; 2], bool, Option<[f32;4]>)> = vec![].to_vec();
        self.current_generation += 1;
        println!("Generating gen {:?}", self.current_generation);

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                match self.apply_rules(x as usize, y as usize) {
                    Some((position, state, color)) => cells_to_change.push((position, state, color)),
                    None => (),
                }
            }
        }

        for (position, state, color) in cells_to_change {
            self.toggle_cell(position, Some(state), color);
        }

        self.current_timer = self.fps;
    }

    fn apply_rules(&mut self, x: usize, y: usize) -> Option<([usize; 2], bool, Option<[f32;4]>)> {
        let neighbour_count = self.get_neighbour_count(x, y);
        if neighbour_count > 0 {
            println!("{:?}, {:?}", [x, y], neighbour_count);
        }

        match (neighbour_count, self.grid[y][x].alive) {
            (c, true) if c < 2 => Some(([y, x], false, None)),
            (2 | 3, true) => None,
            (c, true) if c > 3 => Some(([y, x], false, None)),
            (3, false) => Some(([y, x], true, Some(get_random_color()))),
            _ => None,
        }
    }

    pub fn get_neighbour_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        let upper_limmit_y = min(y + 2, GRID_SIZE as usize);
        let upper_limmit_x = min(x + 2, GRID_SIZE as usize);
        let lower_limit_y: usize = match max(y as isize - 1, 0).try_into() {
            Ok(l) => l,
            Err(_) => 0 as usize,
        };
        let lower_limit_x = match max(x as isize - 1, 0).try_into() {
            Ok(l) => l,
            Err(_) => 0,
        };

        for j in lower_limit_y..upper_limmit_y {
            for i in lower_limit_x..upper_limmit_x {
                if x == i && y == j {
                    continue;
                }

                match self.grid[j][i].alive {
                    true => count += 1,
                    _ => (),
                }
            }
        }
        count
    }

    pub fn clear_board(&mut self) {
        self.grid = vec![vec![Cell::new(); GRID_SIZE as usize]; GRID_SIZE as usize];
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
