/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use rand::distributions::Range;

/// Fitness function for the evolutionary computation benchmark
/// problems evaluated on the hybercube (solution).
///
/// # [Problems](https://www.cs.cmu.edu/afs/cs/project/jair/pub/volume24/ortizboyer05a-html/node6.html)
///
/// * Schwefel: x* = (420.9687, ...), f(x*) = 0
pub fn fitness(problem: Problem, solution: &[f64]) -> f64 {
    match problem {
        Problem::Schwefel => {
            418.9829_f64 * solution.len() as f64
                + solution.iter().fold(0_f64, |sum, x| {
                    sum + x * x.abs().sqrt().sin()
                })
        }
    }
}

/// Domain for the given problem.
pub fn domain(problem: Problem) -> Range<f64> {
    match problem {
        Problem::Schwefel => Range::new(-512.03_f64, 511.97_f64)
    }
}
