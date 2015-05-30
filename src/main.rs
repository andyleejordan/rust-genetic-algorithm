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
    selection: usize,
    elitism: usize,
    mutation: f64,
    crossover: f64,
    verbosity: usize
}

/// Setup and run algorithm to search for solution
fn main() {
    let matches = App::new("rust-genetic-algorithm")
        .version(&crate_version!()[..])
        .author("Andrew Schwartzmeyer <andrew@schwartzmeyer.com>")
        .about("A genetic algorithm in Rust for various benchmark problems.")
        .arg(Arg::with_name("tolerance").short("t").long("tol").takes_value(true)
             .help("Sets the convergence tolerance (0.01)"))
        .arg(Arg::with_name("dimension").short("d").long("dim").takes_value(true)
             .help("Sets the dimension of the hypercube (30)"))
        .arg(Arg::with_name("population").short("p").long("pop").takes_value(true)
             .help("Sets the size of the population (512)"))
        .arg(Arg::with_name("iterations").short("i").long("iter").takes_value(true)
             .help("Sets maximum number of generations (10,000)"))
        .arg(Arg::with_name("selection").short("s").long("select").takes_value(true)
             .help("Sets the size of the tournament selection (4)"))
        .arg(Arg::with_name("elitism").short("e").long("elite").takes_value(true)
             .help("Sets the number of elite replacements (2)"))
        .arg(Arg::with_name("mutation").short("m").long("mut").takes_value(true)
             .help("Sets the chance of mutation (0.8)"))
        .arg(Arg::with_name("crossover").short("c").long("cross").takes_value(true)
             .help("Sets the chance of crossover (0.8)"))
        .arg(Arg::with_name("verbose").short("v").long("verbose").multiple(true)
             .help("Print fitness (1) and solution (2) every 10th generation"))
        .arg(Arg::with_name("problem").multiple(true)
             .help("The problems to solve:
                    * Ackley
                    * Griewangk
                    * Rastrigin
                    * Rosenbrock
                    * Schwefel
                    * Sphere"))
        .get_matches();

    let problems = value_t!(matches.values_of("problem"), Problem)
        .unwrap_or(vec![Problem::Ackley, Problem::Griewangk, Problem::Rastrigin,
                        Problem::Rosenbrock, Problem::Schwefel, Problem::Sphere]);

    let parameters = Parameters {
        tolerance: value_t!(matches.value_of("tolerance"), f64).unwrap_or(0.01_f64),
        dimension: value_t!(matches.value_of("dimension"), usize).unwrap_or(30),
        population: value_t!(matches.value_of("population"), usize).unwrap_or(512),
        iterations: value_t!(matches.value_of("iterations"), usize).unwrap_or(10000),
        selection: value_t!(matches.value_of("selection"), usize).unwrap_or(4),
        elitism: value_t!(matches.value_of("elitism"), usize).unwrap_or(2),
        mutation: value_t!(matches.value_of("mutation"), f64).unwrap_or(0.8_f64),
        crossover: value_t!(matches.value_of("crossover"), f64).unwrap_or(0.8_f64),
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
