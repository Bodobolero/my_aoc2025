
#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input04.txt");

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

pub fn part1() -> u32 {
    let lines = INPUT.lines();
    let height = lines.count() + 2;
    let mut lines = INPUT.lines();
    let width = lines.next().unwrap().as_bytes().iter().count() + 2;
    let mut grid = vec![vec![b'.'; width]; height];
    lines = INPUT.lines();
    for row in 1..height-1 {
        let bytes = lines.next().unwrap().as_bytes();
        for column in 1..width -1 {
            grid[row][column]=bytes[column-1];
        }
    }
    let mut movable_rolls = 0u32;
    for row in 1..height-1 {
        for column in 1..width-1 {
            if grid[row][column]==b'@' {
                let mut rolls = 0usize;
                for dir in DIRECTIONS {
                    if grid[(row as i32 + dir.0) as usize][(column as i32 + dir.1) as usize]== b'@' {
                        rolls+=1;
                        if rolls > 3 {
                            break; 
                        }
                    }
                }
                if rolls < 4 {
                    movable_rolls+=1;
                } 
            }
        }
    }
    movable_rolls
}

pub fn part2() -> u32 {
    let lines = INPUT.lines();
    let height = lines.count() + 2;
    let mut lines = INPUT.lines();
    let width = lines.next().unwrap().as_bytes().iter().count() + 2;
    let mut grid = vec![vec![b'.'; width]; height];
    lines = INPUT.lines();
    for row in 1..height-1 {
        let bytes = lines.next().unwrap().as_bytes();
        for column in 1..width -1 {
            grid[row][column]=bytes[column-1];
        }
    }
    let mut movable_rolls = 0u32;

    let mut touched: bool = true;
    while touched {
        touched = false;
    for row in 1..height-1 {
        for column in 1..width-1 {
            if grid[row][column]==b'@' {
                let mut rolls = 0usize;
                for dir in DIRECTIONS {
                    if grid[(row as i32 + dir.0) as usize][(column as i32 + dir.1) as usize]== b'@' {
                        rolls+=1;
                        if rolls > 3 {
                            break; 
                        }
                    }
                }
                if rolls < 4 {
                    movable_rolls+=1;
                    grid[row][column]=b'.';
                    touched = true;
                } 
            }
        }
    }

    }
    
    movable_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 1493);
    }
    
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 9194);
    }
}

