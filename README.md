Genetic Algorithm
=================

[![Build Status](https://travis-ci.org/andschwa/rust-genetic-algorithm.svg?branch=master)](https://travis-ci.org/andschwa/rust-genetic-algorithm)

A genetic algorithm in [Rust][] for the following [benchmark problems][]:

* Ackley
* Griewangk
* Rastrigin
* Rosenbrock
* Schwefel
* Sphere

Usage:

1. Install [Rust][]
2. Build with `cargo build --release`
3. Search with `./target/release/rust-genetic-algorithm`
4. See further usage with `rust-genetic-algorithm --help`

Based on my prior implementation in [C++][].

[benchmark problems]: https://www.cs.cmu.edu/afs/cs/project/jair/pub/volume24/ortizboyer05a-html/node6.html
[Rust]: http://www.rust-lang.org/
[C++]: https://github.com/andschwa/uidaho-cs472-project1
