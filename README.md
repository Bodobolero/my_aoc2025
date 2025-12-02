# Solutions for Advent of code 2025

This year I solved day 1-12.
The total runtime for part1 and part2 of days 1-11 is xxx milliseconds.

## How to use

Copy input for day into inputs/inputxx.txt
Implement solution in src/days/dayxx.rs

```bash
# test solution for a day
cargo test --lib day01
# run solution for a day
cargo run --bin day01
# run benchmark for a day
cargo bench day01
```

## Benchmarks

| day  |  part1 |  part2 |
| :----|-------:|-------:|
| day01|   98 µs |   99 µs |
| day02|    3 µs | 1877 µs |
| day03|  xxx µs |  xxx µs |
| day04|  xxx µs |  xxx µs |
| day05|  xxx µs |  xxx µs |
| day06|  xxx µs |  xxx µs |
| day07|  xxx µs |  xxx µs |
| day08|  xxx µs |  xxx µs |
| day09|  xxx µs |  xxx µs |
| day10|  xxx µs |  xxx µs |
| day11|  xxx µs |  xxx µs |
| day12|  xxx µs |  xxx µs |


## Benchmark details

Macbook Pro M1 Pro
Apple M1 Pro, 16 GB RAM

```
Running benches/benchmark.rs (target/release/deps/benchmark-b717763455b967e0)
Gnuplot not found, using plotters backend
day01_part1             time:   [97.499 µs 97.652 µs 97.818 µs]
                        change: [+30676309% +30744788% +30808767%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

day01_part2             time:   [98.666 µs 98.852 µs 99.081 µs]
                        change: [+31217200% +31325520% +31435583%] (p = 0.00 < 0.05)
                        Performance has regressed.

Running benches/benchmark.rs (target/release/deps/benchmark-b717763455b967e0)
Gnuplot not found, using plotters backend
day02_part1             time:   [3.0180 µs 3.0227 µs 3.0285 µs]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

Benchmarking day02_part2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.5s, enable flat sampling, or reduce sample count to 50.
day02_part2             time:   [1.8758 ms 1.8771 ms 1.8786 ms]
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low mild
  8 (8.00%) high mild
  7 (7.00%) high severe


```