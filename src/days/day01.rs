use std::panic;

#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input01.txt");

pub fn part1() -> i32 {
    let mut dial: i32 = 50;
    let mut password: i32 = 0;
    for (direction, clicks) in INPUT.lines().map(|line| {
        let (direction, clicks) = line.split_at(1);
        (direction, clicks.parse::<i32>().unwrap())
    }) {
        match direction {
            "L" => {
                dial -= clicks;
                while dial < 0 {
                    dial += 100;
                }
            }
            "R" => {
                dial += clicks;
                while dial > 99 {
                    dial -= 100;
                }
            }
            _ => panic!("invalid direction"),
        }
        if dial == 0 {
            password += 1;
        };
        // println!("The dial is rotated {} in direction {} to point at {}.",clicks, direction, dial );
    }
    password
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
