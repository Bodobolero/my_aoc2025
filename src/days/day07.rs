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

    fn move_beam(
        ys: usize,
        xs: i32,
        linevec: &mut Vec<Vec<u8>>,
        worlds: &mut Vec<Vec<usize>>,
    ) -> usize {
        if xs < 0 || xs >= linevec[0].len() as i32 {
            return 0;
        }
        let x = xs as usize;
        if ys == linevec.len() {
            return 1;
        }
        match linevec[ys][x] {
            b'.' => {
                linevec[ys][x] = b'|';
                let w = move_beam(ys + 1, xs, linevec, worlds);
                worlds[ys][x] = w;
                return w;
            }
            b'^' => {
                let w1 = move_beam(ys + 1, xs - 1, linevec, worlds);
                let w2 = move_beam(ys + 1, xs + 1, linevec, worlds);
                //worlds[ys][x] = w1 + w2;
                return w1 + w2;
            }
            b'|' => return worlds[ys][x],
            _ => panic!("unexpected symbol {}", linevec[ys][x]),
        }
    }
    move_beam(
        1,
        linevec[0].iter().position(|&c| c == b'S').unwrap() as i32,
        &mut linevec,
        &mut worlds,
    )
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
