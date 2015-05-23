#![feature(core)]

extern crate rand;

use std::cmp::{Eq, PartialEq, Ordering, PartialOrd};
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

        let solution = Vec::from_iter((0..n).map(|_| range.ind_sample(&mut rng)));
        let fitness = schwefel(&solution);

        Individual { solution: solution, fitness: fitness }
    }
}

impl Eq for Individual {}

impl Ord for Individual {
    /// This dangerously delegates to `partial_cmp`
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq<Individual> for Individual {
    /// This delegates to `eq` on `fitness: f64`
    fn eq(&self, other: &Individual) -> bool {
        self.fitness.eq(&other.fitness)
    }
}

impl PartialOrd for Individual {
    /// This delegates to `partial_cmp` on `fitness: f64`
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

fn main() {
    let pop_size = 128;

    // initialize population
    let population = Vec::from_iter((0..pop_size).map(|_| Individual::new()));

    let best = population.iter().min_by(|x| schwefel(&x.solution) as i64).unwrap();
    println!("The best solution was {:?}", best.solution);
    println!("Its fitness was {}", best.fitness);
}
