/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Parameters;
use Problem;
use individual::Individual;
use rand::{Rng, thread_rng};
use std::thread;
use time::precise_time_s;

/// A genetic algorithm that searches for convergence to the given
/// tolerance for the problem across the n-dimensional hypercube,
/// using a population of individuals, up to a maximum iterations
/// number of generations.
pub fn search(problem: Problem, parameters: Parameters)
              -> Option<(Individual, usize, f64)> {
    // get thread local random number generator
    let mut rng = thread_rng();

    // initialize population of individuals
    let mut population: Vec<_> = (0..parameters.population).map(|_| {
        Individual::new(problem, parameters.dimension, &mut rng)
    }).collect();

    // start timing the search
    let start_time = precise_time_s();

    // search iterations number of generations
    for i in 0..parameters.iterations {
        // select, mutate, and recombine individuals for next generation
        let mut offspring: Vec<Individual> = Vec::with_capacity(population.len());
        for _ in 0..population.len()/2 {
            let (mut x, mut y) = (select(&population, &mut rng),
                                  select(&population, &mut rng));
            x.mutate(&mut rng);
            y.mutate(&mut rng);
            Individual::combine(&mut x, &mut y, &mut rng);
            offspring.push(x);
            offspring.push(y);
        }
        assert!(offspring.len() == population.len());

        // replace 2 random individuals with elite of prior generation
        for _ in 0..2 {
            if let Some(x) = population.iter().min() {
                offspring[rng.gen_range(0, population.len())] = x.clone();
            }
        }

        // replace population with next generation
        population = offspring;

        // examine best individual for convergence
        if let Some(x) = population.iter().min() {
            if x.fitness < parameters.tolerance {
                return Some((x.clone(), i, precise_time_s() - start_time));
            }
            // print verbose information
            if parameters.verbosity > 0 && i % 10 == 0 {
                let fitness = x.fitness;
                let solution = x.solution.clone();
                thread::spawn(move || {
                    if parameters.verbosity >= 1 {
                        println!("{}th fitness {}", i, fitness);
                    }
                    if parameters.verbosity >= 2 {
                        println!{"{:?}", solution};
                    }
                });
            }
        }
    }
    None
}

/// Tournament selection from 4 random individuals
fn select<R: Rng>(population: &[Individual], rng: &mut R) -> Individual {
    if let Some(selected) = (0..4).map(|_| rng.choose(population)).min() {
        selected.unwrap().clone()
    } else {
        unimplemented!();
    }
}
