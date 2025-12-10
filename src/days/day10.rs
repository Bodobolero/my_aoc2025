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

pub fn part2() -> i64 {
    //[#.#.] (0,2) (2) (2,3) (1,3) {18,16,34,25}
    // we represent the joltage as a vector of integers where each parsed number represents one vector element
    // we represent the buttons as vectors of decimal integers of same length as the joltage that have a 1 in the element positio of the given numbers
    // when computing the joltage by pressing buttons we just need to vector add the button vectors the number of
    // button presses until the result equals the joltage
    // basically the algorithm requires solving a linear equation but the solution vector must only contain positive integers
    // s0 * button0 + s1 * button1 + ... + sn * buttonn = joltage
    // where [s0, s1, ..., sn] is the solution vector with si being the number of presses of button i
    INPUT
        .lines()
        .map(|l| {
            let pos1 = l.find(']').unwrap();
            let pos2 = l.find('{').unwrap();
            let buttons = &l[(pos1 + 2)..(pos2 - 1)];
            let joltage = &l[pos2 + 1..l.len() - 1];
            // print!(
            //     "machine with joltage -{}- and buttons -{}-",
            //     joltage, buttons
            // );
            let target = joltage
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let veclen = target.len();
            // print!(" target {:?} ", target);
            let buttons = buttons
                .split_ascii_whitespace()
                .map(|b| {
                    let mut button = vec![0i64; veclen];
                    for elem in b[1..b.len() - 1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                    {
                        button[elem] = 1;
                    }
                    button
                })
                .collect::<Vec<_>>();
            //println!(" buttons {:?}", buttons);
            (target, buttons)
        })
        .map(|(target, buttons)| {
            let solution = minimal_nonneg_integer_solution(&buttons, &target).unwrap();
            //println!("  solution: {:?}", solution);
            solution.iter().sum::<i64>()
        })
        .sum()
}

fn add(a: &mut [u64], b: &[u64], mult: u64) {
    for i in 0..a.len() {
        a[i] += b[i] * mult;
    }
}

// this solution is too slow, we need to find a solution that systematically solves the integer linear problem (ILP)
pub fn find_minimal_presses(target: &[u64], buttons: &[Vec<u64>]) -> usize {
    // find vector of max presses individually for each button
    let mut max_presses = vec![0; buttons.len()];
    for (i, button) in buttons.iter().enumerate() {
        let mut max = 0usize;
        for (j, val) in button.iter().enumerate() {
            if *val == 1 && target[j] as usize > max {
                max = target[j] as usize;
            }
        }
        max_presses[i] = max;
    }
    //println!(" maxpresses {:?}", max_presses);
    let mut solutions: Vec<Vec<usize>> = Vec::new();
    let minpresses = *target.iter().max().unwrap() as usize;
    let maxtotal = target.iter().sum::<u64>();
    //println!(" minpresses {}", minpresses);
    let ranges: Vec<_> = max_presses.iter().map(|&m| 0..=m).collect();
    for combo in ranges.into_iter().multi_cartesian_product() {
        let presses = combo.iter().sum::<usize>();
        if presses >= minpresses && presses <= maxtotal as usize {
            let mut result = vec![0; target.len()];
            for i in 0..combo.len() {
                add(&mut result, &buttons[i], combo[i] as u64);
            }
            if result == target {
                //println!("  valid combo {:?}", combo);
                solutions.push(combo);
            }
        }
    }

    solutions
        .iter()
        .map(|v| v.iter().sum::<usize>())
        .min()
        .unwrap()
}

// using good_lp solver for integer linear programming
use good_lp::{
    Expression,
    Solution,
    SolverModel,
    constraint,
    solvers::microlp::microlp, // <-- add this
    variable,
    variables,
};

fn minimal_nonneg_integer_solution(basis: &[Vec<i64>], rhs: &[i64]) -> Option<Vec<i64>> {
    let m = rhs.len();
    let n = basis.len();

    if n == 0 || m == 0 {
        return None;
    }
    if basis.iter().any(|v| v.len() != m) {
        return None;
    }

    let mut vars = variables!();
    let xs: Vec<_> = (0..n)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = xs.iter().cloned().sum();

    // use microlp instead of default_solver / CBC
    let mut problem = vars.minimise(objective).using(microlp);

    for i in 0..m {
        let expr: Expression = xs
            .iter()
            .enumerate()
            .map(|(j, xj)| (basis[j][i] as f64) * (*xj))
            .sum();
        problem = problem.with(constraint!(expr == rhs[i] as f64));
    }

    let solution = problem.solve().ok()?;

    let result: Vec<i64> = xs
        .iter()
        .map(|xj| solution.value(*xj).round() as i64)
        .collect();

    Some(result)
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
        assert_eq!(part2(), 17820);
    }
}
