/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use rand::Rng;
use rand::distributions::IndependentSample;
use std::cmp::{Eq, PartialEq, Ordering, PartialOrd};
use std::mem;

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
            problem.domain_dist().ind_sample(rng)
        }).collect();
        let fitness = problem.fitness(&x);
        Individual { solution: x, fitness: fitness, problem: problem }
    }

    /// Mutate with chance n a single gene to a new value in the
    /// problem's domain (a "jump" mutation).
    ///
    /// Fitness is NOT evaluated as it is ALWAYS done in `crossover()`
    pub fn mutate<R: Rng>(&mut self, chance: f64, rng: &mut R) {
        if rng.gen_range(0_f64, 1_f64) < chance {
            let i = rng.gen_range(0, self.solution.len());
            self.solution[i] = self.problem.domain_dist().ind_sample(rng);
        }
    }

    /// Performs two-point crossover with chance n to swap a random
    /// set of [0, dimension] genes between a pair of individuals.
    ///
    /// Fitness is ALWAYS evaluated because it is NOT done in mutate()
    pub fn crossover<R: Rng>(x: &mut Individual, y: &mut Individual,
                             chance: f64, rng: &mut R) {
        assert_eq!(x.problem, y.problem);
        if rng.gen_range(0_f64, 1_f64) < chance {
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
