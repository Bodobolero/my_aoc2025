#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input12.txt");

//#[derive(Debug)]
struct Present {
    unit_size: u64,
    shape: [u64; 3],
}

//#[derive(Debug)]
struct Region {
    dim: [u64; 2],
    presents: Vec<u64>,
}

// we represent each present as bits in a 3 u64 integers.
// we represent each region as bits in n u64 integers (max region widht or height is < 64)
// we can place a present by shifting it to the left n bits and or'ing it to the region
// bit pattern
// optimization to reduce search space
// pre-compute all orientations of each present ? 4 rotations * 2 flips = 8
// sum up the bits of all presents and check if it is smaller than the region space
pub fn part1() -> u32 {
    let mut presents: Vec<Present> = Vec::with_capacity(6);
    let mut regions: Vec<Region> = Vec::with_capacity(1024);

    for part in INPUT.split("\n\n") {
        if part.contains('x') {
            for line in part.lines() {
                let mut parts = line.split(':');
                let mut dims = parts
                    .next()
                    .unwrap()
                    .split('x')
                    .map(|s| s.parse::<u64>().unwrap());
                regions.push(Region {
                    dim: [dims.next().unwrap(), dims.next().unwrap()],
                    presents: parts
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect(),
                });
            }
        } else {
            // present
            let mut p = Present {
                unit_size: 0,
                shape: [0, 0, 0],
            };
            for (i, line) in part.lines().enumerate() {
                if i > 0 {
                    for (j, c) in line.chars().enumerate() {
                        let val = if c == '#' { 1u64 } else { 0u64 };
                        if val == 1 {
                            p.shape[i - 1] += val << j;
                            p.unit_size += 1;
                        }
                    }
                }
            }
            presents.push(p);
        }
    }
    // println!("presents: {:?}", presents);
    // println!("regions: {:?}", regions);
    // let mut count_optimizations = 0u32;
    let mut fits = 0u32;
    for region in regions {
        // optimization: sum up the bits of all presents and check if it is smaller
        // than the region space
        // optimization: sum up the number of presents and check if they fit the region
        // without overlapping
        let mut all_presents_min_size = 0u64;
        let mut all_presents_max_size = 0u64;
        for (i, p_count) in region.presents.iter().enumerate() {
            all_presents_min_size += p_count * presents[i].unit_size;
            all_presents_max_size += p_count * 9;
        }
        if all_presents_min_size > region.dim[0] * region.dim[1] {
            // count_optimizations += 1;
            continue;
        }
        if all_presents_max_size
            <= (region.dim[0] - (region.dim[0] % 3)) * (region.dim[1] - (region.dim[1] % 3))
        {
            fits += 1;
            continue;
        }
        // now to the hard part, fitting all presents
        panic!("we have one that we really need to optimize")
    }
    // println!(
    //     "{} regions are smaller than the sum of all needed pixels",
    //     count_optimizations
    // );
    // println!("{} regions fit without overlaps", fits);
    fits
}

pub fn part2() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 517);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 0);
    }
}
