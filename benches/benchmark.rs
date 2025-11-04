// benches/benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion};

// Macro to generate benchmark functions for each day
macro_rules! generate_benchmarks {
    ($($day:ident),*) => {
        $(
            mod $day {
                use super::*;
                use my_aoc2025::days::$day::{part1, part2};

                pub fn benchmark(c: &mut Criterion) {
                    c.bench_function(concat!(stringify!($day), "_part1"), |b| {
                        b.iter(part1)
                    });
                    c.bench_function(concat!(stringify!($day), "_part2"), |b| {
                        b.iter(part2)
                    });
                }
            }
        )*

        // Collect all benchmarks into a single group
        pub fn benchmarks(c: &mut Criterion) {
            $(
                $day::benchmark(c);
            )*
        }
    };
}

// Use the macro to generate benchmarks for each day
generate_benchmarks!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);

criterion_group!(benches, benchmarks);
criterion_main!(benches);
