extern crate rand;

use rand::thread_rng;
use rand::distributions::Range;
use individual::Individual;

mod individual;

fn main() {
    let mut rng = thread_rng();
    let range = Range::new(-512.03_f64, 511.97); // range for Schwefel problem

    // initialize population
    let population: Vec<_> = (0..128).map(|_| {
        Individual::new(&range, &mut rng)
    }).collect();

    let best = population.iter().min().unwrap();
    println!("The best solution was {:?}", best.solution);
    println!("Its fitness was {}", best.fitness);
}
