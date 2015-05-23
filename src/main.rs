use std::iter::FromIterator;
use individual::Individual;

mod individual;

fn main() {
    let pop_size = 128;

    // initialize population
    let population = Vec::from_iter((0..pop_size).map(|_| Individual::new()));

    let best = population.iter().min().unwrap();
    println!("The best solution was {:?}", best.solution);
    println!("Its fitness was {}", best.fitness);
}
