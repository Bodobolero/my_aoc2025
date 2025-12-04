use std::collections::HashSet;

#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input02.txt");

pub fn sum_invalid_ids_in_range(start: &str, stop: &str) -> u64 {
    // println!("Search IDS for range {} - {}",start, stop);
    let mut min = start.parse::<u64>().unwrap();
    let mut min_len = start.len();
    let mut max = stop.parse::<u64>().unwrap();
    let mut max_len = stop.len();
    let mut ids = 0;
    if !min_len.is_multiple_of(2) {
        min = 10u64.pow(start.len() as u32);
        min_len += 1;
    }
    if !max_len.is_multiple_of(2) {
        max = 10u64.pow(stop.len() as u32 - 1) - 1;
        max_len -= 1;
    }
    if min <= max {
        // println!("   searching invalid IDS in adjusted range {} - {}", min, max);
        let mut min_prefix = min / 10u64.pow(min_len as u32 / 2);
        let max_prefix = max / 10u64.pow(max_len as u32 / 2);
        // min_prefix = min_prefix.max(min_suffix);
        // max_prefix = max_prefix.min(max_suffix);
        // println!("   min {} {} max {} {}", min_prefix, min_suffix, max_prefix, max_suffix);
        while min_prefix <= max_prefix {
            let len = min_prefix.checked_ilog10().map(|x| x + 1).unwrap();
            let id = min_prefix * 10u64.pow(len) + min_prefix;
            if id >= min && id <= max {
                // println!("   found ID {}", id);
                ids += id;
            }
            min_prefix += 1;
        }
        ids
    } else {
        // println!("   no IDS found");
        ids
    }
}

pub fn part1() -> u64 {
    INPUT
        .split(',')
        .map(|s| {
            let mut split = s.split('-');
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(start, stop)| sum_invalid_ids_in_range(start, stop))
        .sum()
}

pub fn sum_invalid_ids_in_range_4_part2(start: &str, stop: &str) -> u64 {
    // println!("Search IDS for range {} - {}",start, stop);
    let min = start.parse::<u64>().unwrap();
    let min_len = start.len();
    let max = stop.parse::<u64>().unwrap();
    let max_len = stop.len();
    let mut ids = 0;
    let mut idset: HashSet<u64> = HashSet::with_capacity(1024);
    for pattern_length in 1..=max_len / 2 {
        for id_length in min_len..=max_len {
            if id_length > 1 && id_length.is_multiple_of(pattern_length) {
                for pattern in
                    10u64.pow(pattern_length as u32 - 1)..10u64.pow(pattern_length as u32)
                {
                    // repeat pattern id_length/pattern_length times
                    let mut id: u64 = 0;
                    for multiplier in (0..id_length).step_by(pattern_length) {
                        id += pattern * 10u64.pow(multiplier as u32);
                    }
                    if id >= min && id <= max && !idset.contains(&id) {
                        // println!("   found ID {}", id);
                        ids += id;
                        idset.insert(id);
                    }
                }
            }
        }
    }
    ids
}

pub fn part2() -> u64 {
    INPUT
        .split(',')
        .map(|s| {
            let mut split = s.split('-');
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(start, stop)| sum_invalid_ids_in_range_4_part2(start, stop))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 38437576669);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 49046150754);
    }
}
