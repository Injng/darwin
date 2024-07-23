/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Structs and functions for the game logic.
 **/

use rand::Rng;
use crate::{ROWS, COLS};

#[derive(Copy, Clone)]
pub struct Chromosome {
    pub strength: u32,
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub chromosome: Chromosome,
}

impl Entity {
    pub fn new(x: u32, y: u32, chromosome: Chromosome) -> Entity {
        Entity { x, y, chromosome }
    }

    // randomly move left, right, up, or down depending on location on grid
    pub fn move_entity(&mut self) {
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

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub entities: Vec<Entity>,
}

/// Initialize entities on the cell grid
pub fn init_entities(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) {
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut rng = rand::thread_rng();
            let spawn = rng.gen_range(0..4);
            if spawn == 1 {
                let mut rng = rand::thread_rng();
                let strength = rng.gen_range(0..100);
                let cell = &mut cells[i as usize][j as usize];
                let chromosome = Chromosome { strength };
                let entity = Entity::new(cell.x, cell.y, chromosome);
                cell.entities.push(entity);
            }
        }
    }
}

/// Play a turn on the cell grid
pub fn play(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) {
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

            // make the strongest entity win
            let mut strongest: &Entity = &cell.entities[0];
            for entity in &cell.entities {
                if entity.chromosome.strength > strongest.chromosome.strength {
                    strongest = entity;
                } else if entity.chromosome.strength == strongest.chromosome.strength {
                    let mut rng = rand::thread_rng();
                    let winner = rng.gen_range(0..2);
                    if winner == 1 { strongest = entity; }
                }
            }
            cell.entities = vec![*strongest];
        }
    }
}

