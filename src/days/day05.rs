use intervaltree::IntervalTree;
const INPUT: &str = include_str!("../../inputs/input05.txt");

pub fn part1() -> usize {
    let mut parts = INPUT.split("\n\n");
    let ranges = parts.next().unwrap();
    let ids = parts.next().unwrap();
    let tree: IntervalTree<u64, ()> = ranges
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            (
                parts.next().unwrap().parse::<u64>().unwrap()
                    ..parts.next().unwrap().parse::<u64>().unwrap() + 1,
                (),
            )
        })
        .collect();
    // println!("tree {:?}", tree);
    // println!("ids {:?}", ids);
    ids.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| tree.query_point(*id).count() > 0)
        .count()
}

pub fn part2() -> u64 {
    let mut parts = INPUT.split("\n\n");
    let ranges = parts.next().unwrap();
    let tree: IntervalTree<u64, ()> = ranges
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            (
                parts.next().unwrap().parse::<u64>().unwrap()
                    ..parts.next().unwrap().parse::<u64>().unwrap() + 1,
                (),
            )
        })
        .collect();
    let mut fresh_id_count = 0u64;
    let mut maxid = 0u64;
    for item in tree.iter_sorted() {
        let mut start = item.range.start;
        if maxid > item.range.end {
            continue;
        }
        if maxid > start {
            start = maxid;
        }
        fresh_id_count += item.range.end - start;
        maxid = item.range.end;
    }
    fresh_id_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 868);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 354143734113772);
    }
}
