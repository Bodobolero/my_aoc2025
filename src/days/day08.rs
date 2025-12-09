const INPUT: &str = include_str!("../../inputs/input08.txt");
use ordered_float::OrderedFloat;
use std::collections::{BinaryHeap, HashSet};

fn euclidean_distance_3d(a: &(f64, f64, f64), b: &(f64, f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn part1() -> usize {
    let n = 1000usize;
    // read all points into a vector
    let points: Vec<(f64, f64, f64)> = INPUT
        .lines()
        .map(|line| {
            let mut s = line.split(',');
            (
                s.next().unwrap().parse::<f64>().unwrap(),
                s.next().unwrap().parse::<f64>().unwrap(),
                s.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect();

    // keep a binary (max) heap of n=1000 shortest distances
    // the elements of the heap are: distance: f64, index_of_point1: usize, index_of_point2: usize, indexes are into points vector
    let mut top_n_shortest_distances: BinaryHeap<(OrderedFloat<f64>, usize, usize)> =
        BinaryHeap::with_capacity(n);
    // compare all points for n = 1000 we have ca 500.000 comparisions (n^2/2)
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let distance = OrderedFloat(euclidean_distance_3d(&p1, &p2));
            // we always compare the new distance to the max of the existing up to n shortest distances
            if top_n_shortest_distances.len() < n {
                top_n_shortest_distances.push((distance, i, j));
            } else if let Some(&current_max) = top_n_shortest_distances.peek()
                && distance < current_max.0
            {
                // if the new one is smaller we keep it
                top_n_shortest_distances.pop();
                top_n_shortest_distances.push((distance, i, j));
            }
        }
    }
    // now we have the n shortest distances in the heap, but unsorted
    // connect them into sets of junction boxes, for now I think it is not necessary to sort the pairs by distances
    // we can use the junction box indices as set elements
    // so we create a vector of sets of usize indices
    let mut sets: Vec<HashSet<usize>> = Vec::new();
    for pair in top_n_shortest_distances {
        let i1 = pair.1;
        let i2 = pair.2;
        let mut set1: Option<usize> = None;
        let mut set2: Option<usize> = None;
        // we need to find the sets that contain each junction box
        for (i, set) in sets.iter().enumerate() {
            if set.contains(&i1) {
                set1 = Some(i);
                break;
            }
        }
        for (i, set) in sets.iter().enumerate() {
            if set.contains(&i2) {
                set2 = Some(i);
                break;
            }
        }
        match (set1, set2) {
            // if both are in the same set we do nothing
            (Some(a), Some(b)) if a == b => {}
            // if both are in different sets we merge the sets
            (Some(a), Some(b)) => {
                if a < b {
                    let s1 = sets.remove(b);
                    sets[a].extend(s1);
                } else {
                    let s1 = sets.remove(a);
                    sets[b].extend(s1);
                }
            }
            // if only one is contained in a set we add the other one to the set
            (Some(a), None) => {
                sets[a].insert(i2);
            }
            (None, Some(b)) => {
                sets[b].insert(i1);
            }
            // if both are in no set we create a new set
            (None, None) => {
                let mut newset: HashSet<usize> = HashSet::new();
                newset.insert(i1);
                newset.insert(i2);
                sets.push(newset);
            }
        }
    }
    sets.sort_unstable_by_key(|set2| std::cmp::Reverse(set2.len()));
    sets[0].len() * sets[1].len() * sets[2].len()
}

pub fn part2() -> usize {
    // read all points into a vector
    let points: Vec<(f64, f64, f64)> = INPUT
        .lines()
        .map(|line| {
            let mut s = line.split(',');
            (
                s.next().unwrap().parse::<f64>().unwrap(),
                s.next().unwrap().parse::<f64>().unwrap(),
                s.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .collect();
    let n = points.len();

    // compute all distances and store them in a vector
    // the elements of the vec are: distance: f64, index_of_point1: usize, index_of_point2: usize, indexes are into points vector
    let mut distances: Vec<(OrderedFloat<f64>, usize, usize)> = Vec::with_capacity(n * n / 2);
    // compare all points for n = 1000 we have ca 500.000 comparisions (n^2/2)
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let distance = OrderedFloat(euclidean_distance_3d(&p1, &p2));
            distances.push((distance, i, j));
        }
    }
    distances.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", distances);
    // now we have the n shortest distances in the heap, but unsorted
    // connect them into sets of junction boxes, for now I think it is not necessary to sort the pairs by distances
    // we can use the junction box indices as set elements
    // so we create a vector of sets of usize indices
    let mut sets: Vec<HashSet<usize>> = Vec::new();
    for pair in distances {
        let i1 = pair.1;
        let i2 = pair.2;
        let mut set1: Option<usize> = None;
        let mut set2: Option<usize> = None;
        // we need to find the sets that contain each junction box
        for (i, set) in sets.iter().enumerate() {
            if set.contains(&i1) {
                set1 = Some(i);
                break;
            }
        }
        for (i, set) in sets.iter().enumerate() {
            if set.contains(&i2) {
                set2 = Some(i);
                break;
            }
        }
        match (set1, set2) {
            // if both are in the same set we do nothing
            (Some(a), Some(b)) if a == b => {}
            // if both are in different sets we merge the sets
            (Some(a), Some(b)) => {
                if a < b {
                    let s1 = sets.remove(b);
                    sets[a].extend(s1);
                } else {
                    let s1 = sets.remove(a);
                    sets[b].extend(s1);
                }
            }
            // if only one is contained in a set we add the other one to the set
            (Some(a), None) => {
                sets[a].insert(i2);
            }
            (None, Some(b)) => {
                sets[b].insert(i1);
            }
            // if both are in no set we create a new set
            (None, None) => {
                let mut newset: HashSet<usize> = HashSet::new();
                newset.insert(i1);
                newset.insert(i2);
                sets.push(newset);
            }
        }
        if sets.len() == 1 && sets[0].len() == n {
            return points[i1].0 as usize * points[i2].0 as usize;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 90036);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 6083499488);
    }
}
