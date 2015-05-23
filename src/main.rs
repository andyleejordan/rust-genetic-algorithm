extern crate rand;

use std::iter::FromIterator;
use rand::distributions::{IndependentSample, Range};

const N: usize = 50;
const P: usize = 128;

fn schwefel(solution: &Vec<f64>) -> f64 {
    418.9829_f64 * solution.len() as f64
        + solution.iter().fold(0_f64, |sum, x| sum + x * x.abs().sqrt().sin())
}

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(-512.03, 511.97);

    // initialize population
    let population = Vec::from_iter((0..P).map(|_| {
        Vec::from_iter((0..N).map(|_| {
            range.ind_sample(&mut rng)
        }))
    }));

    println!("{:?}", population); // debug print of solutions

    println!("The best solution's fitness was {}", schwefel(&population[0]));
}
