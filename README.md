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
| day03|   31 µs |   68 µs |
| day04|  145 µs | 1500 µs |
| day05|  119 µs |   18 µs |
| day06|  133 µs |   27 µs |
| day07|   34 µs |   61 µs | 
| day08| 4681 µs |25321 µs |
| day09|  xxx µs |  xxx µs |
| day10|  xxx µs |  xxx µs |
| day11|  xxx µs |  xxx µs |
| day12|  xxx µs |  xxx µs |

(Python day07 part2, same solution: 1941 microseconds = 32 times slower than Rust)

## Benchmark details

Macbook Pro M1 Pro
Apple M1 Pro, 16 GB RAM

```bash
rustc --version
rustc 1.93.0-nightly (20383c9f1 2025-11-03)
```


```bash
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
     Running benches/benchmark.rs (target/release/deps/benchmark-b717763455b967e0)
Gnuplot not found, using plotters backend
day03_part1             time:   [30.621 µs 30.647 µs 30.676 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

day03_part2             time:   [68.226 µs 68.402 µs 68.582 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

     Running benches/benchmark.rs (target/release/deps/benchmark-c84cec2f19a55fbf)
Gnuplot not found, using plotters backend
day04_part1             time:   [143.26 µs 145.37 µs 147.41 µs]
                        change: [−3.1389% −0.8811% +1.4632%] (p = 0.45 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low mild

Benchmarking day04_part2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.6s, enable flat sampling, or reduce sample count to 50.
day04_part2             time:   [1.4948 ms 1.4998 ms 1.5049 ms]
                        change: [−5.9877% −5.5774% −5.1936%] (p = 0.00 < 0.05)
                        Performance has improved.

     Running benches/benchmark.rs (target/release/deps/benchmark-c84cec2f19a55fbf)
Gnuplot not found, using plotters backend
day05_part1             time:   [118.49 µs 118.71 µs 118.94 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

day05_part2             time:   [17.800 µs 17.806 µs 17.813 µs]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

     Running benches/benchmark.rs (target/release/deps/benchmark-c84cec2f19a55fbf)
Gnuplot not found, using plotters backend
day06_part1             time:   [132.53 µs 132.77 µs 133.03 µs]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

day06_part2             time:   [26.892 µs 26.935 µs 26.978 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

     Running benches/benchmark.rs (target/release/deps/benchmark-c84cec2f19a55fbf)
Gnuplot not found, using plotters backend
day07_part1             time:   [34.431 µs 34.612 µs 34.798 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

day07_part2             time:   [59.621 µs 59.892 µs 60.220 µs]

day07 recursive:

     Running benches/benchmark.rs (target/release/deps/benchmark-c84cec2f19a55fbf)
Gnuplot not found, using plotters backend
day07_part1             time:   [33.663 µs 33.814 µs 33.996 µs]
                        change: [−2.2419% −1.3136% −0.5067%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

day07_part2             time:   [61.036 µs 61.190 µs 61.363 µs]
                        change: [−7.4529% −7.2069% −6.9656%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

     Running benches/benchmark.rs (target/release/deps/benchmark-89ebb62f4c2c57cd)
Gnuplot not found, using plotters backend
day08_part1             time:   [4.6774 ms 4.6806 ms 4.6842 ms]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

day08_part2             time:   [25.305 ms 25.321 ms 25.340 ms]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  10 (10.00%) high severe


```