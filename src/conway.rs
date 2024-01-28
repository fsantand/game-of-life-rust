use std::{
    cmp::{max, min},
    isize, usize,
};

pub const GRID_SIZE: i8 = 40;
pub const CELL_SIZE: f64 = 32.0;
pub const TIME_BETWEEN_GENERATIONS: f64 = 0.3;

#[derive(Debug)]
pub struct State {
    pub grid: Vec<Vec<bool>>,
    pub running: bool,
    pub fps: f64,
    pub current_timer: f64,
    pub current_generation: u32,
}

impl State {
    pub fn new() -> State {
        State {
            grid: vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize],
            running: false,
            fps: TIME_BETWEEN_GENERATIONS,
            current_timer: TIME_BETWEEN_GENERATIONS,
            current_generation: 0,
        }
    }

    pub fn toggle_cell(&mut self, position: [usize; 2], is_alive: Option<bool>) -> () {
        match is_alive {
            Some(state) => self.grid[position[0]][position[1]] = state,
            None => self.grid[position[0]][position[1]] = !self.grid[position[0]][position[1]],
        }
        println!(
            "Cell at {:?} changed to {:?}",
            position, self.grid[position[0]][position[1]]
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
        let mut cells_to_change: Vec<([usize;2], bool)> = vec![].to_vec();
        self.current_generation += 1;
        println!("Generating gen {:?}", self.current_generation);

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                match self.apply_rules(x as usize, y as usize) {
                    Some((position, state)) => cells_to_change.push((position, state)),
                    None => (),
                }
            }
        }

        for (position, state) in cells_to_change {
            self.toggle_cell(position, Some(state));
        }

        self.current_timer = self.fps;
    }

    fn apply_rules(&mut self, x: usize, y: usize) -> Option<([usize;2], bool)> {
        let neighbour_count = self.get_neighbour_count(x, y);
        if neighbour_count > 0 {
            println!("{:?}, {:?}", [x, y], neighbour_count);
        }

        match (neighbour_count, self.grid[y][x]) {
            (c, true) if c < 2 => Some(([y, x], false)),
            (2 | 3, true) => None,
            (c, true) if c > 3 => Some(([y, x], false)),
            (3, false) => Some(([y, x], true)),
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

                match self.grid[j][i] {
                    true => count += 1,
                    _ => (),
                }
            }
        }
        count
    }
}
