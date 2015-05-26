/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use std::cmp::{Eq, PartialEq, Ordering, PartialOrd};
use std::mem;
use rand::Rng;
use rand::distributions::IndependentSample;

/// An Orderable, Cloneable solution with a cached fitness
#[derive(Clone)]
pub struct Individual {
    pub solution: Vec<f64>,
    pub fitness: f64,
    problem: Problem,
}

impl Individual {
    /// Constructs a new Individual to solve Problem with n random values
    pub fn new<R: Rng>(problem: Problem, dimension: usize, rng: &mut R) -> Self {
        let x: Vec<_> = (0..dimension).map(|_| {
            problem.domain().ind_sample(rng)
        }).collect();
        let fitness = problem.fitness(&x);
        Individual { solution: x, fitness: fitness, problem: problem }
    }

    /// Mutates 0 or 1 random genes to a new value in the range
    /// Fitness is NOT evaluated as it is ALWAYS done in `combine()`
    pub fn mutate<R: Rng>(&mut self, rng: &mut R) {
        for _ in 0..rng.gen_range(0, 2) {
            let i = rng.gen_range(0, self.solution.len());
            self.solution[i] = self.problem.domain().ind_sample(rng);
        }
    }

    /// Recombines two Individuals with a 1 in 2 chance via two-point crossover
    /// Fitness is ALWAYS evaluated because it is NOT done in mutate()
    pub fn combine<R: Rng>(x: &mut Individual, y: &mut Individual, rng: &mut R) {
        assert_eq!(x.problem, y.problem);
        if rng.gen_weighted_bool(2) {
            let len = x.solution.len();
            let (start, n) = (rng.gen_range(0, len), rng.gen_range(0, len));
            for i in start..start + n {
                mem::swap(&mut x.solution[i % len], &mut y.solution[i % len]);
            }
        }
        x.fitness = x.problem.fitness(&x.solution);
        y.fitness = y.problem.fitness(&y.solution);
    }
}

impl Eq for Individual {}

impl Ord for Individual {
    /// This dangerously delegates to `partial_cmp`; NaN will panic
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
