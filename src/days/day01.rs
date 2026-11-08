use std::panic;

#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input01.txt");

pub fn part1() -> usize {
    let mut pos = 50;
    INPUT
        .lines()
        .map(|line| {
            let sign = if line.starts_with('L') { -1 } else { 1 };
            let val = line[1..].parse::<i32>().unwrap();
            pos += sign * val;
            if pos < 0 {
                pos += 100
            };
            pos %= 100;
            pos
        })
        .filter(|pos| *pos == 0)
        .count()
}

pub fn part2() -> i32 {
    let mut dial: i32 = 50;
    let mut password: i32 = 0;
    for (direction, clicks) in INPUT.lines().map(|line| {
        let (direction, clicks) = line.split_at(1);
        (direction, clicks.parse::<i32>().unwrap())
    }) {
        match direction {
            "L" => {
                password += clicks / 100;
                let clicks2 = clicks % 100;
                if clicks2 > 0 {
                    dial -= clicks2;
                    if dial < 0 {
                        if dial + clicks2 != 0 {
                            password += 1;
                        }
                        dial += 100;
                    }
                    if dial == 0 {
                        password += 1;
                    }
                }
            }
            "R" => {
                dial += clicks;
                password += dial / 100;
                dial = dial % 100;
            }
            _ => panic!("invalid direction"),
        }
        // println!("The dial is rotated {} in direction {} to point at {} password is {}.",clicks, direction, dial, password );
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 1007);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 5820);
    }
}
