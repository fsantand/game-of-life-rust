pub const GRID_SIZE: i8 = 40;
pub const CELL_SIZE: f64 = 16.0;

#[derive(Debug)]
pub struct State {
    pub grid: Vec<Vec<bool>>,
    pub running: bool,
}

impl State {
    pub fn new() -> State {
        State {
            grid: vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize],
            running: false,
        }
    }

    pub fn toggle_cell(&mut self, position: [usize;2]) -> () {
        self.grid[position[0]][position[1]] = !self.grid[position[0]][position[1]];
        println!("Cell at {:?} changed to {:?}", position, self.grid[position[0]][position[1]]);
    }
}
