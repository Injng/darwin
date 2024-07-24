/**
 * Copyright (c) 2024, Lin Jiang (@Injng)
 *
 * Functions for the evolution of entities.
 **/

use rand::distributions::{Distribution, Uniform};
use crate::game::Entity;

#[derive(Copy, Clone)]
pub struct Chromosome {
    pub strength: u32,
    pub aggressive: u32,
    pub agility: u32,
}

// mutate a chromosome
fn mutate(genes: Chromosome) -> Chromosome {
    // calculate changes
    let bounds = Uniform::from((-10 as i32)..10);
    let mut rng = rand::thread_rng();
    let add_strength = bounds.sample(&mut rng);
    let add_aggressive = bounds.sample(&mut rng);
    let add_agility = bounds.sample(&mut rng);

    // apply changes
    let strength = (genes.strength as i32 + add_strength).max(0).min(100) as u32;
    let aggressive = (genes.aggressive as i32 + add_aggressive).max(0).min(100) as u32;
    let agility = (genes.agility as i32 + add_agility).max(0).min(100) as u32;
    Chromosome { strength, aggressive, agility }
}

// given a set of chromosomes, evolve more by using random mutations
pub fn evolve(entities: Vec<Entity>, n: usize) -> Vec<Chromosome> {
    let mut chromosomes: Vec<Chromosome> = Vec::new();
    for entity in entities {
        for _ in 0..n {
            let chromosome = mutate(entity.chromosome);
            chromosomes.push(chromosome);
        }
    }
    chromosomes
}

