/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

#[macro_use]
extern crate clap;
extern crate rand;
extern crate time;

use clap::App;

mod algorithm;
mod individual;
mod problem;

arg_enum!{
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Problem {
        Ackley,
        Schwefel
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
             [verbose]... -v 'Print fitness (1) and solution (2) every 10th generation'")
        .get_matches();
    let problem = value_t!(matches.value_of("problem"), Problem)
        .unwrap_or(Problem::Schwefel);
    let dimension = value_t!(matches.value_of("d"), usize).unwrap_or(30);
    let population = value_t!(matches.value_of("p"), usize).unwrap_or(256);
    let iterations = value_t!(matches.value_of("i"), usize).unwrap_or(5000);
    let verbosity = matches.occurrences_of("verbose") as usize;

    algorithm::search(problem, 0.05_f64, dimension, population, iterations, verbosity)
}
