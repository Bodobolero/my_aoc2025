use std::collections::HashSet;

use itertools::Itertools;
const INPUT: &str = include_str!("../../inputs/input10.txt");

pub fn part1() -> usize {
    // we model pressing buttons as XOR'ing binary integers
    // we need to press each button 0 or 1 times because
    // A XOR B XOR B = A
    // double pressing cancels out each other, thus multiples of 2 or zero operations and pressing same button all odd numbers are like pressing it once
    // possible optimizations:
    // - reduce duplicate buttons (a button is an unordered set of bit positions and is duplicate if it contains the same bit(positions))
    // we use u64 datatype for our patters
    // set nth it
    // let mut x: 0u64;
    // x |= 1 << n;
    // xor:  x ^ y

    INPUT
        .lines()
        .map(|l| {
            let pos1 = l.find(']').unwrap();
            let diagram = &l[1..pos1];
            let buttons = &l[(pos1 + 2)..(l.find('{').unwrap() - 1)];
            let mut target = 0u64;
            for (i, c) in diagram.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        target += 1 << i;
                    }
                    _ => panic!("invalid char"),
                }
            }
            let button_set = buttons
                .split_ascii_whitespace()
                .map(|b| {
                    let mut button = 0u64;
                    for i in b[1..b.len() - 1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                    {
                        button += 1 << i;
                    }
                    button
                })
                .collect::<HashSet<u64>>();
            (target, button_set)
        })
        .map(|(target, buttons)| {
            for presses in 1..=buttons.len() {
                for subset in buttons.iter().combinations(presses) {
                    let mut result = 0u64;
                    for button in subset {
                        result ^= button;
                    }
                    if result == target {
                        return presses;
                    }
                }
            }
            panic!(" no combination of buttons found")
        })
        .sum()
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let pos1 = l.find(']').unwrap();
            let diagram = &l[1..pos1];
            let buttons = &l[(pos1 + 2)..(l.find('{').unwrap() - 1)];
            print!(
                "machine with diagram -{}- and buttons -{}-",
                diagram, buttons
            );
            let mut target = 0u64;
            for (i, c) in diagram.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        target += 1 << i;
                    }
                    _ => panic!("invalid char"),
                }
            }
            print!(" target {} ", target);
            let button_set = buttons
                .split_ascii_whitespace()
                .map(|b| {
                    let mut button = 0u64;
                    for i in b[1..b.len() - 1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                    {
                        button += 1 << i;
                    }
                    button
                })
                .collect::<HashSet<u64>>();
            println!(" buttons {:?}", button_set);
            (target, button_set)
        })
        .map(|(target, buttons)| {
            let mut presses = 0usize;
            for presses in 1..=buttons.len() {
                for subset in buttons.iter().combinations(presses) {
                    let mut result = 0u64;
                    for button in subset {
                        result ^= button;
                    }
                    if result == target {
                        return presses;
                    }
                }
            }
            panic!(" no combination of buttons found")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 486);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 33);
    }
}
