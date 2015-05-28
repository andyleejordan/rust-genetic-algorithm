/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

#[macro_use]
extern crate clap;
extern crate rand;
extern crate time;

use clap::{App, Arg};
use std::thread;

mod algorithm;
mod individual;
mod problem;

arg_enum!{
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Problem {
        Ackley,
        Griewangk,
        Rastrigin,
        Rosenbrock,
        Schwefel,
        Sphere
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
        .author("Andrew Schwartzmeyer <andrew@schwartzmeyer.com>")
        .about("A genetic algorithm in Rust for various benchmark problems.")
        .arg(Arg::with_name("tolerance").short("t").long("tol").takes_value(true)
             .help("Sets the convergence tolerance (0.05)"))
        .arg(Arg::with_name("dimension").short("d").long("dim").takes_value(true)
             .help("Sets the dimension of the hypercube (30)"))
        .arg(Arg::with_name("population").short("p").long("pop").takes_value(true)
             .help("Sets the size of the population (256)"))
        .arg(Arg::with_name("iterations").short("i").long("iter").takes_value(true)
             .help("Sets maximum number of generations (5000)"))
        .arg(Arg::with_name("verbose").short("v").long("verbose").multiple(true)
             .help("Print fitness (1) and solution (2) every 10th generation"))
        .arg(Arg::with_name("problem").multiple(true)
             .help("The problems to solve: Schwefel or Ackley"))
        .get_matches();

    let problems = value_t!(matches.values_of("problem"), Problem)
        .unwrap_or(vec![Problem::Ackley, Problem::Griewangk, Problem::Rastrigin,
                        Problem::Rosenbrock, Problem::Schwefel, Problem::Sphere]);

    let parameters = Parameters {
        tolerance: value_t!(matches.value_of("tolerance"), f64).unwrap_or(0.05_f64),
        dimension: value_t!(matches.value_of("dimension"), usize).unwrap_or(30),
        population: value_t!(matches.value_of("population"), usize).unwrap_or(256),
        iterations: value_t!(matches.value_of("iterations"), usize).unwrap_or(5000),
        verbosity: matches.occurrences_of("verbose") as usize
    };

    let workers = problems.iter().map(|&problem| {
        thread::spawn(move || algorithm::search(problem, parameters))
    });

    for worker in workers {
        if let Ok(results) = worker.join() {
            println!("{} converged to {} after {} generations in {} seconds.",
                     results.problem, results.individual.fitness,
                     results.iterations, results.duration);
            println!("{:?}", results.individual.solution);
        }
    }
}
