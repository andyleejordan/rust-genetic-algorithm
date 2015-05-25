extern crate rand;

use rand::Rng;
use rand::distributions;
use individual::Individual;

mod individual;

fn select<R: Rng>(population: &Vec<Individual>, rng: &mut R)
                  -> Individual {
    if let Some(selected) = (0..4).map(|_| rng.choose(population)).min() {
        selected.unwrap().clone()
    } else {
        unimplemented!();
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let range = distributions::Range::new(-512.03_f64, 511.97);

    // initialize population
    let mut population: Vec<_> = (0..128).map(|_| {
        Individual::new(&range, &mut rng)
    }).collect();

    for i in 0..10000 {
        // select and mutate individuals for next population
        population = (0..128).map(|_| {
            select(&population, &mut rng).mutate(&range, &mut rng)
        }).collect();

        if let Some(x) = population.iter().min() {
            if i % 100 == 0 {
                println!("{}th fitness: {}", i, x.fitness);
            }

            if x.fitness < 1000_f64 {
                println!("{}th solution converged at {}: {:?}",
                         i, x.fitness, x.solution);
                return;
            }
        } else { unimplemented!() }
    }
    println!("Failed to converge.");
}
