use core::f64;
use std::usize;

use conway::{State, CELL_SIZE, GRID_SIZE};
use input::{InputHandler, PlayerActions};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    EventLoop, EventSettings, Events, Key, MouseButton, MouseCursorEvent, PressEvent,
    RenderEvent, WindowSettings, UpdateEvent,
};
use piston_window::{clear, color, rectangle, Context, PistonWindow, Transformed};

mod conway;
mod input;

const CELL_RECT: [f64; 4] = [0.0, 0.0, CELL_SIZE as f64, CELL_SIZE as f64];

fn paint_board(state: &State, c: Context, g: &mut GlGraphics) {
    for j in 0..GRID_SIZE {
        for i in 0..GRID_SIZE {
            match state.grid[j as usize][i as usize] {
                true => rectangle(
                    color::WHITE,
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

fn transform_to_grid_coordinates(position: [f64;2]) ->[usize;2] {
    [(position[1]/CELL_SIZE) as usize, (position[0]/CELL_SIZE) as usize]
}

fn main() {
    let opengl = OpenGL::V4_0;
    let mut window: PistonWindow = WindowSettings::new("The Game of Life", (512, 512))
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
    input_handler.add_mapping(Key::P, PlayerActions::PreviousStep);

    input_handler.add_click_mapping(MouseButton::Left, PlayerActions::ToggleTile);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            cursor = args;
        }

        if let Some(args) = e.press_args() {
            match input::handle_input(&input_handler, args, cursor) {
                PlayerActions::RunSimulation => todo!(),
                PlayerActions::NextStep => todo!(),
                PlayerActions::PreviousStep => todo!(),
                PlayerActions::ToggleTile => game.toggle_cell(transform_to_grid_coordinates(cursor)),
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
        }
    }
}
