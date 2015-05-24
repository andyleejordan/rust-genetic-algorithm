extern crate rand;

use rand::thread_rng;
use rand::distributions::Range;
use individual::Individual;

mod individual;

fn main() {
    let mut rng = thread_rng();
    let range = Range::new(-512.03_f64, 511.97); // range for Schwefel problem

    // initialize population
    let mut population: Vec<_> = (0..128).map(|_| {
        Individual::new(&range, &mut rng)
    }).collect();

    for i in 0..10000 {
        // generate mutated offspring
        population = population.iter().map(|x| {
            x.mutate(&range, &mut rng)
        }).collect();

        let best = population.iter().min().unwrap();
        if i % 100 == 0 {
            println!("{}th fitness: {}", i, best.fitness);
        }

        if best.fitness < 1000_f64 {
            println!("Solution: {:?}", best.solution);
            return;
        }
    }
    println!("Failed to converge.");
}
