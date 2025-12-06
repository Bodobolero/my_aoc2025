#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input06.txt");

pub fn part1() -> u64 {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut sum = 0u64;
    for (linenr, line) in INPUT.lines().enumerate() {
        let symbols = line.split_ascii_whitespace();
        if line.starts_with('*') || line.starts_with('+') {
            for (symbolnr, symbol) in symbols.enumerate() {
                match symbol {
                    "+" => {
                        sum += numbers[symbolnr].iter().sum::<u64>();
                    }
                    "*" => {
                        sum += numbers[symbolnr].iter().product::<u64>();
                    }
                    _ => panic!("invalid operator"),
                }
            }
        } else {
            for (symbolnr, symbol) in symbols.enumerate() {
                if linenr == 0 {
                    let mut v = Vec::with_capacity(1024);
                    v.push(symbol.parse::<u64>().unwrap());
                    numbers.push(v);
                } else {
                    numbers[symbolnr].push(symbol.parse::<u64>().unwrap());
                }
            }
        }
    }
    sum
}

pub fn part2() -> u64 {
    let linevec = INPUT.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    let digits = linevec.len() - 1;
    let operators = linevec.last().unwrap();
    let mut sum = 0u64;
    for (i, operator) in operators.iter().enumerate() {
        match *operator {
            b'+' | b'*' => {
                let mut summand = 0u64;
                if *operator == b'*' {
                    summand = 1;
                }
                // search index of next operator or end and derive end of operand from it
                let mut j = i + 1;
                while j < operators.len() && operators[j] == b' ' {
                    j += 1;
                }
                if j == operators.len() {
                    j -= 1;
                } else {
                    j -= 2;
                }
                // println!("parsing operands from pos {} to {}", i, j);
                for pos in i..=j {
                    let mut operand = 0u64;
                    for l in 0..digits {
                        let digit = linevec[l][pos];

                        if digit != b' ' {
                            operand *= 10;
                            operand += (digit - b'0') as u64;
                        }
                    }
                    // println!("  operand {}", operand);
                    if *operator == b'*' {
                        summand *= operand;
                    } else {
                        summand += operand;
                    }
                }
                sum += summand;
            }
            b' ' => {
                // do nothing
            }
            _ => panic!("invalid operator"),
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 5877594983578);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 11159825706149);
    }
}
