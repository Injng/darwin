/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Main file that initializes the SDL2 window and event loop.
 **/

pub mod evolution;
pub mod game;
pub mod theme;

use evolution::evolve;
use game::{Cell, Entity, init_entities, play};

use core::array::from_fn;
use sdl2::event::Event;
use sdl2::render::Canvas;
use std::thread::sleep;
use std::time::Duration;

const ROWS: u32 = 20;
const COLS: u32 = 32;
const OFFSET: u32 = 5;

/// Draw a grid on the canvas
fn draw_grid(canvas: &mut Canvas<sdl2::video::Window>, width: u32, height: u32, cols: u32, rows: u32) {
    let cell_width = width / cols;
    let cell_height = height / rows;
    canvas.set_draw_color(theme::FOREGROUND);
    for i in 0..rows {
        canvas.draw_line((0, (i * cell_height) as i32), (width as i32, (i * cell_height) as i32)).unwrap();
    }
    for i in 0..cols {
        canvas.draw_line(((i * cell_width) as i32, 0), ((i * cell_width) as i32, height as i32)).unwrap();
    }
}

/// Render entity on the canvas
fn draw_entity(canvas: &mut Canvas<sdl2::video::Window>, entity: Entity, offset: u32) {
    let grid_width = 1440 / COLS;
    let grid_height = 900 / ROWS;
    let x = entity.x * grid_width + offset;
    let y = entity.y * grid_height + offset;
    canvas.set_draw_color(theme::CELL);
    canvas.fill_rect(sdl2::rect::Rect::new(x as i32, y as i32, grid_width - 2 * offset, grid_height - 2 * offset)).unwrap();
}

fn main() {
    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("darwin", 1440, 900)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // initialize grid of cells
    let mut cells: [[Cell; COLS as usize]; ROWS as usize] = from_fn(|_| from_fn(|_| Cell { x: 0, y: 0, entities: Vec::new() }));
    for i in 0..ROWS {
        for j in 0..COLS {
            cells[i as usize][j as usize] = Cell { x: j, y: i, entities: Vec::new() };
        }
    }

    // initialize entities
    init_entities(&mut cells, Vec::new());

    // create event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_paused = false;
    'running: loop {
        if !is_paused {
            // draw things
            let mut entity_count = 0;
            canvas.set_draw_color(theme::BACKGROUND);
            canvas.clear();
            draw_grid(&mut canvas, 1440, 900, COLS, ROWS);
            for i in 0..ROWS {
                for j in 0..COLS {
                    for entity in &cells[i as usize][j as usize].entities {
                        draw_entity(&mut canvas, *entity, OFFSET);
                        entity_count += 1;
                    }
                }
            }
            canvas.present();

            // advance game turns
            sleep(Duration::from_millis(10));
            play(&mut cells);

            // evolve when only four entities remain
            if entity_count <= 4 {
                let mut entities: Vec<Entity> = Vec::new();
                for i in 0..ROWS {
                    for j in 0..COLS {
                        for entity in &cells[i as usize][j as usize].entities {
                            entities.push(*entity);
                        }
                    }
                }
                let chromosomes = evolve(entities, 5);
                init_entities(&mut cells, chromosomes);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                // pause when space is hit
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => {
                    is_paused = !is_paused;
                },
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}

