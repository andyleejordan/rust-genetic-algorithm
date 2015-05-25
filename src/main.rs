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
    let mut population: Vec<_> = (0..512).map(|_| {
        Individual::new(&range, &mut rng)
    }).collect();

    for i in 0..10000 {
        // select, mutate, and recombine individuals for next population
        let mut offspring: Vec<Individual> = Vec::with_capacity(population.len());
        for _ in (0..population.len()/2) {
            let (mut x, mut y) = (select(&population, &mut rng),
                                  select(&population, &mut rng));
            x.mutate(&range, &mut rng);
            y.mutate(&range, &mut rng);
            Individual::combine(&mut x, &mut y, &mut rng);
            offspring.push(x);
            offspring.push(y);
        }
        assert!(offspring.len() == population.len());

        // replace random individuals with elite of prior generation
        for _ in 0..2 {
            if let Some(x) = population.iter().min() {
                offspring[rng.gen_range(0, population.len())] = x.clone();
            }
        }

        population = offspring;

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
