extern crate rand;

use rand::distributions::{IndependentSample, Range};

const N: usize = 50;
const P: usize = 128;

fn schwefel(solution: &[f64; N]) -> f64 {
    418.9829_f64 * N as f64
        + solution.iter().fold(0_f64, |sum, x| sum + x * x.abs().sqrt().sin())
}

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(-512.03, 511.97);

    let mut population = [[0_f64; N]; P];

    for solution in &mut population[..] {
        for x in &mut solution[..] {
            *x = range.ind_sample(&mut rng);
        }
    }

    // debug print of solutions
    for solution in &population[..] {
        for x in &solution[..] {
            println!("{} ", x);
        }
    }
    println!("The best solution's fitness was {}", schwefel(&population[0]));
}
