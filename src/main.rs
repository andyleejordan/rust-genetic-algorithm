extern crate rand;

use rand::Rng;
use rand::distributions;
use individual::Individual;

mod individual;

/// Tournament selection from 4 random individuals
fn select<R: Rng>(population: &Vec<Individual>, rng: &mut R)
                  -> Individual {
    if let Some(selected) = (0..4).map(|_| rng.choose(population)).min() {
        selected.unwrap().clone()
    } else {
        unimplemented!();
    }
}

/// Setup and run algorithm to search for solution
fn main() {
    let mut rng = rand::thread_rng();
    let range = distributions::Range::new(-500_f64, 500_f64);

    // initialize population of 512 individuals
    let mut population: Vec<_> = (0..512).map(|_| {
        Individual::new(&range, &mut rng)
    }).collect();

    // search through at most 10,000 generations
    for i in 0..10000 {
        // select, mutate, and recombine individuals for next generation
        let mut offspring: Vec<Individual> = Vec::with_capacity(population.len());
        for _ in 0..population.len()/2 {
            let (mut x, mut y) = (select(&population, &mut rng),
                                  select(&population, &mut rng));
            x.mutate(&range, &mut rng);
            y.mutate(&range, &mut rng);
            Individual::combine(&mut x, &mut y, &mut rng);
            offspring.push(x);
            offspring.push(y);
        }
        assert!(offspring.len() == population.len());

        // replace 2 random individuals with elite of prior generation
        for _ in 0..2 {
            if let Some(x) = population.iter().min() {
                offspring[rng.gen_range(0, population.len())] = x.clone();
            }
        }

        // replace population with next generation
        population = offspring;

        // examine best individual for convergence
        if let Some(x) = population.iter().min() {
            if i % 1000 == 0 {
                println!("{}th fitness: {}", i, x.fitness);
            }

            if x.fitness < 0.05_f64 {
                println!("{}th solution converged at {}: {:?}",
                         i, x.fitness, x.solution);
                return;
            }
        } else { unimplemented!() }
    }
    println!("Failed to converge.");
}
