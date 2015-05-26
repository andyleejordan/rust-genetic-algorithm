/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use rand::distributions::Range;

impl Problem {
    /// Fitness function for the evolutionary computation benchmark
    /// problems evaluated on the hybercube.
    ///
    /// # [Problems](https://www.cs.cmu.edu/afs/cs/project/jair/pub/volume24/ortizboyer05a-html/node6.html)
    ///
    /// * Schwefel: x* = (420.9687, ...), f(x*) = 0
    pub fn fitness(&self, x: &[f64]) -> f64 {
        match *self {
            Problem::Schwefel => {
                418.9829_f64 * x.len() as f64 + x.iter().fold(0_f64, |sum, i| {
                    sum + i * i.abs().sqrt().sin()
                })
            }
        }
    }

    /// Domain for the given problem.
    pub fn domain(&self) -> Range<f64> {
        match *self {
            Problem::Schwefel => Range::new(-512.03_f64, 511.97_f64)
        }
    }
}
