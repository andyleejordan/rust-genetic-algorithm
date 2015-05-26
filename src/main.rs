/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

#[macro_use]
extern crate clap;
extern crate rand;
extern crate time;

use clap::App;
use rand::Rng;
use rand::distributions;
use time::precise_time_s;
use std::thread;
use individual::Individual;

mod individual;

arg_enum!{
    #[derive(Copy, Clone)]
    pub enum Problem {
        Schwefel
    }
}

/// Tournament selection from 4 random individuals
fn select<R: Rng>(population: &[Individual], rng: &mut R) -> Individual {
    if let Some(selected) = (0..4).map(|_| rng.choose(population)).min() {
        selected.unwrap().clone()
    } else {
        unimplemented!();
    }
}

/// Setup and run algorithm to search for solution
fn main() {
    let matches = App::new("rust-genetic-algorithm")
        .version(&crate_version!()[..])
        .author("Andrew Schwartzmeyer <andrew@schwartzmeyer.com")
        .about("A genetic algorithm in Rust for Schwefel's function.")
        .args_from_usage(
            "[<problem>] 'The problem to solve'
             [-d --dimension <30>] 'Sets the dimension of the hypercube'
             [-p --population <256>] 'Sets the size of the population'
             [-i --iterations <5000>] 'Sets maximum number of generations'
             --verbose 'Print fitness every 10th generation'")
        .get_matches();
    let problem = value_t!(matches.value_of("problem"), Problem)
        .unwrap_or(Problem::Schwefel);
    let dimension = value_t!(matches.value_of("d"), usize).unwrap_or(30);
    let population_size = value_t!(matches.value_of("p"), usize).unwrap_or(256);
    let iterations = value_t!(matches.value_of("i"), usize).unwrap_or(5000);
    let verbose = matches.is_present("verbose");

    let mut rng = rand::thread_rng();
    let range = distributions::Range::new(-500_f64, 500_f64);

    // initialize population individuals
    let mut population: Vec<_> = (0..population_size).map(|_| {
        Individual::new(dimension, &range, &mut rng)
    }).collect();

    let start_time = precise_time_s();
    // search with generations
    for i in 0..iterations {
        // select, mutate, and recombine individuals for next generation
        let mut offspring: Vec<Individual> = Vec::with_capacity(population.len());
        for _ in 0..population.len()/2 {
            let (mut x, mut y) = (select(&population, &mut rng),
                                  select(&population, &mut rng));
            x.mutate(&range, &mut rng);
            y.mutate(&range, &mut rng);
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
            if verbose && i % 10 == 0 {
                let fitness = x.fitness;
                thread::spawn(move || {
                    println!("{}th fitness {}", i, fitness)
                });
            }
            if x.fitness < 0.05_f64 {
                let duration = precise_time_s() - start_time;
                println!("{}th solution converged at {} in {} seconds: {:?}",
                         i, x.fitness, duration, x.solution);
                return;
            }
        } else { unimplemented!() }
    }
    println!("Failed to converge.");
}
