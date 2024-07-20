/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Main file that initializes the SDL2 window and event loop.
 **/

pub mod theme;

use sdl2::event::Event;

/// Draw a grid on the canvas
fn draw_grid(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, width: u32, height: u32, cols: u32, rows: u32) {
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

fn main() {
    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("darwin", 1440, 900)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    // create event loop
    canvas.set_draw_color(theme::BACKGROUND);
    canvas.clear();
    draw_grid(&mut canvas, 1440, 900, 16, 10);
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}

