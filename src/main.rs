/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Main file that initializes the SDL2 window and event loop.
 **/

pub mod theme;

use core::array::from_fn;
use rand::Rng;
use sdl2::event::Event;
use sdl2::render::Canvas;
use std::thread::sleep;
use std::time::Duration;

const ROWS: u32 = 10;
const COLS: u32 = 16;

#[derive(Copy, Clone)]
struct Chromosome {
    strength: bool,
}

#[derive(Copy, Clone)]
struct Entity {
    x: u32,
    y: u32,
    chromosome: Chromosome,
}

impl Entity {
    fn new(x: u32, y: u32, chromosome: Chromosome) -> Entity {
        Entity { x, y, chromosome }
    }

    // randomly move left, right, up, or down depending on location on grid
    fn move_entity(&mut self) {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..4);
        match direction {
            0 => {
                if self.x > 0 {
                    self.x -= 1;
                }
            },
            1 => {
                if self.x < COLS - 1 {
                    self.x += 1;
                }
            },
            2 => {
                if self.y > 0 {
                    self.y -= 1;
                }
            },
            3 => {
                if self.y < ROWS - 1 {
                    self.y += 1;
                }
            },
            _ => {}
        }
    }
}

struct Cell {
    x: u32,
    y: u32,
    entities: Vec<Entity>,
}

/// Initialize entities on the cell grid
fn init_entities(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) {
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut rng = rand::thread_rng();
            let spawn = rng.gen_range(0..4);
            if spawn == 1 {
                let cell = &mut cells[i as usize][j as usize];
                let chromosome = Chromosome { strength: false };
                let entity = Entity::new(cell.x, cell.y, chromosome);
                cell.entities.push(entity);
            }
        }
    }
}

/// Play a turn on the cell grid
fn play(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) {
    // move each entity and remove moved entities from cells
    let mut entities: Vec<Entity> = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            while let Some(entity) = cells[i as usize][j as usize].entities.pop() {
                let mut entity = entity.clone();
                entity.move_entity();
                entities.push(entity);
            }
        }
    }

    // add moved entities back to cells
    for entity in entities {
        cells[entity.y as usize][entity.x as usize].entities.push(entity);
    }

    // handle fights when two entities occupy same cell
    for i in 0..ROWS {
        for j in 0..COLS {
            let cell = &mut cells[i as usize][j as usize];

            // exit early if no conflicting entities in cell
            if cell.entities.len() < 2 {
                continue;
            }

            // only consider the strong entities
            let mut strong: Vec<Entity> = Vec::new();
            for entity in &cell.entities {
                if entity.chromosome.strength {
                    strong.push(*entity);
                }
            }

            // if no strong entities, randomly pick a winner
            if strong.is_empty() {
                let mut rng = rand::thread_rng();
                let winner = rng.gen_range(0..cell.entities.len());
                cell.entities = vec![cell.entities[winner]];
                continue;
            }

            // randomly pick a winner
            if cell.entities.len() > 1 {
                let mut rng = rand::thread_rng();
                let winner = rng.gen_range(0..strong.len());
                cell.entities = vec![cell.entities[winner]];
            }
        }
    }
}

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
    init_entities(&mut cells);

    // create event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_paused = false;
    'running: loop {
        if !is_paused {
            // draw things
            canvas.set_draw_color(theme::BACKGROUND);
            canvas.clear();
            draw_grid(&mut canvas, 1440, 900, COLS, ROWS);
            for i in 0..ROWS {
                for j in 0..COLS {
                    for entity in &cells[i as usize][j as usize].entities {
                        draw_entity(&mut canvas, *entity, 10);
                    }
                }
            }
            canvas.present();

            // advance game turns
            sleep(Duration::from_millis(100));
            play(&mut cells);
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

