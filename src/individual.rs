use std::cmp::{Eq, PartialEq, Ordering, PartialOrd};
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn schwefel(solution: &Vec<f64>) -> f64 {
    418.9829_f64 * solution.len() as f64
        + solution.iter().fold(0_f64, |sum, x| sum + x * x.abs().sqrt().sin())
}

pub struct Individual {
    pub solution: Vec<f64>,
    pub fitness: f64,
}

impl Individual {
    pub fn new<R: Rng>(range: &Range<f64>, rng: &mut R) -> Self {
        let solution = (0..50).map(|_| range.ind_sample(rng)).collect();
        let fitness = schwefel(&solution);

        Individual { solution: solution, fitness: fitness }
    }

    pub fn mutate<R: Rng>(&self, range: &Range<f64>, rng: &mut R) -> Individual {
        let solution = self.solution.iter().map(|&x| {
            // mutate 1 in 40 genes to a new value
            if rng.gen_weighted_bool(40) { range.ind_sample(rng) } else { x }
        }).collect();
        let fitness = schwefel(&solution);
        Individual { solution: solution, fitness: fitness }
    }
}

impl Clone for Individual {
    fn clone(&self) -> Self {
        Individual { solution: self.solution.clone(), fitness: self.fitness }
    }
}

impl Eq for Individual {}

impl Ord for Individual {
    /// This dangerously delegates to `partial_cmp`
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(result) = self.fitness.partial_cmp(&other.fitness) {
            return result;
        }
        unimplemented!();
    }
}

impl PartialEq<Individual> for Individual {
    /// This doesn't use `fitness.eq()` because it needs to be
    /// consistent with `Eq`
    fn eq(&self, other: &Individual) -> bool {
        if let Some(result) = self.fitness.partial_cmp(&other.fitness) {
            return result == Ordering::Equal;
        }
        unimplemented!();
    }
}

impl PartialOrd for Individual {
    /// This delegates to `fitness.partial_cmp()`
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}
