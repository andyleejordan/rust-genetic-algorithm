fn schwefel(solution: &Vec<f64>) -> f64 {
    return 418.9829_f64 * solution.len() as f64
        + solution.iter().fold(0_f64, |sum, x| sum + x * x.abs().sqrt().sin());
}

fn main() {
    println!("Hello, world!");
}
