#![feature(core)]

extern crate rand;

use std::iter::FromIterator;
use rand::distributions::{IndependentSample, Range};

fn schwefel(solution: &Vec<f64>) -> f64 {
    418.9829_f64 * solution.len() as f64
        + solution.iter().fold(0_f64, |sum, x| sum + x * x.abs().sqrt().sin())
}

struct Individual {
    solution: Vec<f64>,
    fitness: f64,
}

impl Individual {
    fn new() -> Individual {
        let n = 50;
        let mut rng = rand::thread_rng();
        let range = Range::new(-512.03_f64, 511.97);

        let mut i = Individual {
            solution: Vec::from_iter((0..n).map(|_| range.ind_sample(&mut rng))),
            fitness: 0_f64,
        };
        i.fitness = schwefel(&i.solution);
        i
    }
}

fn main() {
    let pop_size = 128;

    // initialize population
    let population = Vec::from_iter((0..pop_size).map(|_| Individual::new()));

    let best = population.iter().min_by(|x| schwefel(&x.solution) as i64).unwrap();
    println!("The best solution was {:?}", best.solution);
    println!("Its fitness was {}", schwefel(&best.solution));
}
