use individual::Individual;

mod individual;

fn main() {
    let pop_size = 128;

    // initialize population
    let population: Vec<_> = (0..pop_size).map(|_| Individual::new()).collect();

    let best = population.iter().min().unwrap();
    println!("The best solution was {:?}", best.solution);
    println!("Its fitness was {}", best.fitness);
}
