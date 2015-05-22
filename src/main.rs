const N: usize = 50;

fn schwefel(solution: &[f64; N]) -> f64 {
    return 418.9829_f64 * N as f64
        + solution.iter().fold(0_f64, |sum, x| sum + x * x.abs().sqrt().sin());
}

fn main() {
    println!("Hello, world!");
}
