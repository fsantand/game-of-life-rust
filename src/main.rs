use std::usize;

use conway::{State, CELL_SIZE, GRID_SIZE};
use input::{InputHandler, PlayerActions};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    EventSettings, Events, Key, MouseButton, MouseCursorEvent, PressEvent, RenderEvent,
    UpdateEvent, WindowSettings,
};
use piston_window::{clear, color, rectangle, Context, PistonWindow, Transformed};
use rand::Rng;

mod conway;
mod input;

const CELL_RECT: [f64; 4] = [0.0, 0.0, CELL_SIZE as f64, CELL_SIZE as f64];
const CELL_COLORS: [[f32;4];3] = [color::WHITE, color::CYAN, color::TEAL];

fn get_random_color() ->[f32;4] {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..CELL_COLORS.len() as usize);
    CELL_COLORS[n]
}

fn paint_board(state: &State, c: Context, g: &mut GlGraphics) {
    for j in 0..GRID_SIZE {
        for i in 0..GRID_SIZE {
            let cell = state.grid[j as usize][i as usize];
            match cell.alive {
                true => rectangle(
                    match cell.color {
                        Some(c) => c,
                        None => color::WHITE,
                    },
                    CELL_RECT,
                    c.transform
                        .trans(i as f64 * CELL_SIZE, j as f64 * CELL_SIZE),
                    g,
                ),
                _ => (),
            }
        }
    }
}

fn transform_to_grid_coordinates(position: [f64; 2]) -> [usize; 2] {
    [
        (position[1] / CELL_SIZE) as usize,
        (position[0] / CELL_SIZE) as usize,
    ]
}

fn main() {
    let opengl = OpenGL::V4_0;
    let mut window: PistonWindow = WindowSettings::new("The Game of Life", (1024, 1024))
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);
    let mut cursor = [0.0, 0.0];
    let mut game = State::new();

    let mut input_handler = InputHandler::new();
    input_handler.add_mapping(Key::R, PlayerActions::RunSimulation);
    input_handler.add_mapping(Key::N, PlayerActions::NextStep);

    input_handler.add_click_mapping(MouseButton::Left, PlayerActions::ToggleTile);
    input_handler.add_click_mapping(MouseButton::Right, PlayerActions::CountNeightbours);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            cursor = args;
        }

        if let Some(args) = e.press_args() {
            match input::handle_input(&input_handler, args, cursor) {
                PlayerActions::RunSimulation => game.toggle_simulation(),
                PlayerActions::ToggleTile => {
                    game.toggle_cell(transform_to_grid_coordinates(cursor), None, Some(get_random_color()));
                }
                PlayerActions::CountNeightbours => {
                    let [x, y] = transform_to_grid_coordinates(cursor);
                    println!("{:?}: #{:?}", [x, y], game.get_neighbour_count(y, x));
                }
                PlayerActions::NextStep => {
                    game.generate_next_state();
                }
                _ => (),
            }
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear(color::BLACK, g);

                paint_board(&game, c, g);
            })
        }

        if let Some(args) = e.update_args() {
            if game.run_timer(args.dt) {
                game.generate_next_state();
            }
        }
    }
}
