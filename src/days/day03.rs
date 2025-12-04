#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input03.txt");

fn bank_joltage(xs: &[u8]) -> u32 {
    let mut i_max1 = 0;
    for x in 1..xs.len() - 1 {
        if xs[x] > xs[i_max1] {
            i_max1 = x;
        }
    }
    let mut i_max2 = i_max1 + 1;

    for x in i_max2 + 1..xs.len() {
        if xs[x] > xs[i_max2] {
            i_max2 = x;
        }
    }

    (xs[i_max1] - b'0') as u32 * 10 + (xs[i_max2] - b'0') as u32
}

pub fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| line.as_bytes())
        .map(bank_joltage)
        .sum()
}

fn bank_joltage_part2(xs: &[u8]) -> u64 {
    let mut indices = [0usize; 12];
    let mut i_max1 = 0;
    for x in 1..=xs.len() - 12 {
        if xs[x] > xs[i_max1] {
            i_max1 = x;
        }
    }
    indices[0] = i_max1;

    for i in 1..12 {
        let i_prev = indices[i - 1];
        let mut i_max = i_prev + 1;
        for x in i_max + 1..=xs.len() - 12 + i {
            // print!("i:{}x:{}-", i,x);
            if xs[x] > xs[i_max] {
                i_max = x;
            }
        }
        indices[i] = i_max;
        // print!("->{},",i_max);
    }
    // println!();

    let mut joltage = (xs[indices[0]] - b'0') as u64;
    for i in 1..12 {
        joltage *= 10;
        joltage += (xs[indices[i]] - b'0') as u64;
    }
    // println!("joltage {}", joltage);
    joltage
}

pub fn part2() -> u64 {
    INPUT
        .lines()
        .map(|line| line.as_bytes())
        .map(bank_joltage_part2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 17332);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 172516781546707);
    }
}
