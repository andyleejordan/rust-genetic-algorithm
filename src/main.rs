/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

#[macro_use]
extern crate clap;
extern crate rand;
extern crate time;

use clap::App;
use std::thread;

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

#[derive(Debug, Copy, Clone)]
pub struct Parameters {
    tolerance: f64,
    dimension: usize,
    population: usize,
    iterations: usize,
    verbosity: usize
}

/// Setup and run algorithm to search for solution
fn main() {
    let matches = App::new("rust-genetic-algorithm")
        .version(&crate_version!()[..])
        .author("Andrew Schwartzmeyer <andrew@schwartzmeyer.com")
        .about("A genetic algorithm in Rust for Schwefel's function.")
        .args_from_usage(
            "[problem]... 'The problem to solve'
             -t --tolerance [0.05] 'Sets the convergence tolerance'
             -d --dimension [30] 'Sets the dimension of the hypercube'
             -p --population [256] 'Sets the size of the population'
             -i --iterations [5000] 'Sets maximum number of generations'
             [verbose]... -v 'Print fitness (1) and solution (2) every 10th generation'")
        .get_matches();
    let problems = value_t!(matches.values_of("problem"), Problem)
        .unwrap_or(vec![Problem::Schwefel, Problem::Ackley]);
    let parameters = Parameters {
        tolerance: value_t!(matches.value_of("t"), f64).unwrap_or(0.05_f64),
        dimension: value_t!(matches.value_of("d"), usize).unwrap_or(30),
        population: value_t!(matches.value_of("p"), usize).unwrap_or(256),
        iterations: value_t!(matches.value_of("i"), usize).unwrap_or(5000),
        verbosity: matches.occurrences_of("verbose") as usize
    };

    let workers = problems.iter().map(|&problem| {
        thread::spawn(move || algorithm::search(problem, parameters))
    });

    for worker in workers {
        if let Ok(results) = worker.join() {
            if let Some(individual) = results.individual {
                println!("{} converged to {} after {} generations in {} seconds.",
                         results.problem, individual.fitness,
                         results.iterations, results.duration);
                println!("{:?}", individual.solution);
            } else {
                println!("{} failed to converge.", results.problem);
            }
        }
    }
}
