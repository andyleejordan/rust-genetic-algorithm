/// A genetic algorithm in Rust
/// Copyright (C) 2015  Andrew Schwartzmeyer

use Problem;
use rand::distributions::Range;
use std::f64::consts;

impl Problem {
    /// Fitness function for the evolutionary computation benchmark
    /// problems evaluated on the hybercube of dimension p, where
    /// f(x*) = 0.
    ///
    /// # [Problems](https://www.cs.cmu.edu/afs/cs/project/jair/pub/volume24/ortizboyer05a-html/node6.html)
    ///
    /// * Ackley: 20+e-20*exp(-0.2*sqrt((1/p)*sum(x_i^2)))-exp((1/p)*sum(cos(2*pi*x_i))), x*=(0, ...)
    /// * Griewangk: 1+sum(x_i^2/4000)-prod(cos(x_i/sqrt(i))), x*=(0, ...)
    /// * Rastrigin: 10*p+sum(x_i^2-10*cos(2*pi*x_i)), x*=(0, ...)
    /// * Rosenbrock: sum(100*(x_(i+1)-x_i^2)^2+(x_i-1)^2), x*=(1, ...)
    /// * Schwefel: 418.9829*p+sum(x_i*sin(sqrt(abs(x_i)))), x*=(420.9687, ...)
    /// * Sphere: sum(x_i^2), x*=(0, ...)
    pub fn fitness(&self, x: &[f64]) -> f64 {
        let p = x.len() as f64;
        match *self {
            Problem::Ackley => {
                20_f64 + consts::E - 20_f64 *
                    (-0.2_f64 * (p.recip() * (x.iter().fold(0_f64, |sum, x| {
                        sum + x.powi(2)
                    })).sqrt())).exp() -
                    (p.recip() * x.iter().fold(0_f64, |sum, x| {
                        sum + (2_f64 * consts::PI * x).cos()
                    })).exp()}
            Problem::Griewangk => {
                1_f64 + x.iter().fold(0_f64, |sum, x| {
                    sum + x.powi(2)/4000_f64
                }) - x.iter().enumerate().fold(1_f64, |prod, (i, x)| {
                    prod * (x/((i + 1) as f64).sqrt()).cos()
                })}
            Problem::Rastrigin => {
                10_f64 * p + x.iter().fold(0_f64, |sum, x| {
                    sum + x.powi(2) - 10_f64 * (2_f64 * consts::PI * x).cos()
                })}
            Problem::Rosenbrock => {
                x.iter().skip(1).zip(x).fold(0_f64, |sum, (x_next, x)| {
                    sum + 100_f64 * (x_next - x.powi(2)).powi(2)
                        + (x - 1_f64).powi(2)
                })}
            Problem::Schwefel => {
                418.9829_f64 * p + x.iter().fold(0_f64, |sum, i| {
                    sum + i * i.abs().sqrt().sin()
                })}
            Problem::Sphere => {
                x.iter().fold(0_f64, |sum, x| sum + x.powi(2))}
        }
    }

    /// Domain for the given problem.
    pub fn domain(&self) -> (f64, f64) {
        match *self {
            Problem::Ackley     => (-30_f64, 30_f64),
            Problem::Griewangk  => (-600_f64, 600_f64),
            Problem::Rastrigin  => (-5.12_f64, 5.12_f64),
            Problem::Rosenbrock => (-2.048_f64, 2.048_f64),
            Problem::Schwefel   => (-512.03_f64, 511.97_f64),
            Problem::Sphere     => (-5.12_f64, 5.12_f64),
        }
    }

    /// Random distribution for problem's domain
    pub fn domain_dist(&self) -> Range<f64> {
        let (a, b) = self.domain();
        Range::new(a, b)
    }
}
