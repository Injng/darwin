/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Structs and functions for the game logic.
 **/

use rand::{Rng, distributions::{Distribution, Uniform}};
use std::u32::MAX;
use crate::{ROWS, COLS};
use crate::evolution::Chromosome;

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

    // move left, right, up, and down depending on aggressiveness and agility
    pub fn move_entity(&mut self, distances: &[[f32; COLS as usize]; ROWS as usize], agile: bool) {
        // order the cells by distance in increasing order
        let mut cells: Vec<Cell> = Vec::new();
        if self.x > 0 {
            cells.push(Cell { x: self.x - 1, y: self.y, entities: Vec::new() });
        }
        if self.x < COLS - 1 {
            cells.push(Cell { x: self.x + 1, y: self.y, entities: Vec::new() });
        }
        if self.y > 0 {
            cells.push(Cell { x: self.x, y: self.y - 1, entities: Vec::new() });
        }
        if self.y < ROWS - 1 {
            cells.push(Cell { x: self.x, y: self.y + 1, entities: Vec::new() });
        }
        cells.sort_by(|a, b| {
            let distance_a = distances[a.y as usize][a.x as usize];
            let distance_b = distances[b.y as usize][b.x as usize];
            distance_a.partial_cmp(&distance_b).unwrap()
        });

        // if less than 4 cells in vector, pad to 4 with last cell
        while cells.len() < 4 {
            cells.push(cells[cells.len() - 1].clone());
        }

        // move to the closest cell with a probability based on aggressiveness
        let bounds = Uniform::from(0..100);
        let mut rng = rand::thread_rng();
        let first = bounds.sample(&mut rng);
        let second = bounds.sample(&mut rng);
        let third = bounds.sample(&mut rng);
        if first < self.chromosome.aggressive {
            self.x = cells[0].x;
            self.y = cells[0].y;
        } else if second < self.chromosome.aggressive {
            self.x = cells[1].x;
            self.y = cells[1].y;
        } else if third < self.chromosome.aggressive {
            self.x = cells[2].x;
            self.y = cells[2].y;
        } else {
            self.x = cells[3].x;
            self.y = cells[3].y;
        }

        // if agility is greater than 50, move again without recalculating distances
        if agile && self.chromosome.agility > 50 {
            self.move_entity(&distances, false);
        }
    }
}

#[derive(Clone)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub entities: Vec<Entity>,
}

/// Clear the cell grid of any entities
fn clear_cells(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) {
    for i in 0..ROWS {
        for j in 0..COLS {
            cells[i as usize][j as usize].entities.clear();
        }
    }
}

/// Initialize entities on the cell grid
pub fn init_entities(cells: &mut [[Cell; COLS as usize]; ROWS as usize], chromosomes: Vec<Chromosome>) {
    clear_cells(cells);
    let mut count = 0;
    for i in 0..ROWS {
        for j in 0..COLS {
            let mut rng = rand::thread_rng();
            let spawn = rng.gen_range(0..4);
            if spawn == 1 {
                let chromosome: Chromosome;
                if chromosomes.is_empty() {
                    // if no chromosomes are given, generate random values for genes
                    let bounds = Uniform::from(0..100);
                    let mut rng = rand::thread_rng();
                    let strength = bounds.sample(&mut rng);
                    let aggressive = bounds.sample(&mut rng);
                    let agility = bounds.sample(&mut rng);
                    chromosome = Chromosome { strength, aggressive, agility };
                } else {
                    // otherwise select next chromosome
                    chromosome = chromosomes[count % chromosomes.len()];
                    count += 1;
                }

                // add to entities
                let cell = &mut cells[i as usize][j as usize];
                let entity = Entity::new(cell.x, cell.y, chromosome);
                cell.entities.push(entity);
            }
        }
    }
}

/// Create an array showing the distance from each cell to the nearest entity
pub fn get_distances(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) -> [[f32; COLS as usize]; ROWS as usize] {
    let mut distances: [[f32; COLS as usize]; ROWS as usize] = [[MAX as f32; COLS as usize]; ROWS as usize];
    for i in 0..ROWS {
        for j in 0..COLS {
            if !cells[i as usize][j as usize].entities.is_empty() {
                for k in 0..ROWS {
                    for l in 0..COLS {
                        let distance = ((i as i32 - k as i32).pow(2) + (j as i32 - l as i32).pow(2)) as f32;
                        if distance < distances[k as usize][l as usize] {
                            distances[k as usize][l as usize] = distance;
                        }
                    }
                }
            }
        }
    }
    distances
}

/// Play a turn on the cell grid
pub fn play(cells: &mut [[Cell; COLS as usize]; ROWS as usize]) {
    // move each entity and remove moved entities from cells
    let mut entities: Vec<Entity> = Vec::new();
    for i in 0..ROWS {
        for j in 0..COLS {
            while let Some(entity) = cells[i as usize][j as usize].entities.pop() {
                let distances = get_distances(cells);
                let mut entity = entity.clone();
                entity.move_entity(&distances, true);
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

