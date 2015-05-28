/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use rand::distributions::Range;
use std::f64::consts;

impl Problem {
    /// Fitness function for the evolutionary computation benchmark
    /// problems evaluated on the hybercube.
    ///
    /// # [Problems](https://www.cs.cmu.edu/afs/cs/project/jair/pub/volume24/ortizboyer05a-html/node6.html)
    ///
    /// * Schwefel: x* = (420.9687, ...), f(x*) = 0
    pub fn fitness(&self, x: &[f64]) -> f64 {
        match *self {
            Problem::Ackley => {
                let p = x.len() as f64;
                20_f64 + consts::E - 20_f64 *
                    (-0.2_f64 * (p.recip() * (x.iter().fold(0_f64, |sum, i| {
                        sum + i.powi(2)
                    })).sqrt())).exp() -
                    (p.recip() * x.iter().fold(0_f64, |sum, i| {
                        sum + (2_f64 * consts::PI * i).cos()
                    })).exp()
            }
            Problem::Griewangk => {
                1_f64 + x.iter().fold(0_f64, |sum, x| {
                    sum + x.powi(2)/4000_f64
                }) - x.iter().enumerate().fold(1_f64, |prod, (i, x)| {
                    prod * (x/((i + 1) as f64).sqrt()).cos()
                })
            }
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
            Problem::Ackley => Range::new(-30_f64, 30_f64),
            Problem::Schwefel => Range::new(-512.03_f64, 511.97_f64),
            Problem::Griewangk => Range::new(-600_f64, 600_f64),
        }
    }
}
