const INPUT: &str = include_str!("../../inputs/input07.txt");

pub fn part1() -> usize {
    let mut linevec = INPUT
        .lines()
        .map(|l| l.as_bytes().to_vec()) // now you own the bytes
        .collect::<Vec<Vec<u8>>>();
    let mut splits = 0usize;
    for linenr in 0..linevec.len() - 1 {
        let linelen = linevec[linenr].len();
        for i in 0..linelen {
            match linevec[linenr][i] {
                b'S' => linevec[linenr + 1][i] = b'|',
                b'|' => match linevec[linenr + 1][i] {
                    b'.' => linevec[linenr + 1][i] = b'|',
                    b'^' => {
                        splits += 1;
                        if i > 0 {
                            linevec[linenr + 1][i - 1] = b'|'
                        }
                        if i < linelen - 1 {
                            linevec[linenr + 1][i + 1] = b'|'
                        }
                    }
                    b'|' => {}
                    _ => panic!("unexpected symbol {}", linevec[linenr][i]),
                },
                b'.' | b'^' => {}
                _ => panic!("unexpected symbol"),
            }
        }
    }
    splits
}

pub fn part2() -> usize {
    let mut linevec = INPUT
        .lines()
        .map(|l| l.as_bytes().to_vec()) // now you own the bytes
        .collect::<Vec<Vec<u8>>>();
    let mut worlds: Vec<Vec<usize>> = vec![vec![0; linevec[0].len()]; linevec.len()];
    for linenr in 0..linevec.len() - 1 {
        let linelen = linevec[linenr].len();
        for i in 0..linelen {
            match linevec[linenr][i] {
                b'S' => {
                    linevec[linenr + 1][i] = b'|';
                    worlds[linenr + 1][i] = 1;
                }
                b'|' => match linevec[linenr + 1][i] {
                    b'.' => {
                        linevec[linenr + 1][i] = b'|';
                        worlds[linenr + 1][i] += worlds[linenr][i];
                    }
                    b'^' => {
                        if i > 0 {
                            linevec[linenr + 1][i - 1] = b'|';
                            worlds[linenr + 1][i - 1] += worlds[linenr][i];
                        }
                        if i < linelen - 1 {
                            linevec[linenr + 1][i + 1] = b'|';
                            worlds[linenr + 1][i + 1] += worlds[linenr][i];
                        }
                    }
                    b'|' => {
                        worlds[linenr + 1][i] += worlds[linenr][i];
                    }
                    _ => panic!("unexpected symbol {}", linevec[linenr][i]),
                },
                b'.' | b'^' => {}
                _ => panic!("unexpected symbol"),
            }
        }
    }
    worlds.last().unwrap().iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 1630);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 47857642990160);
    }
}
