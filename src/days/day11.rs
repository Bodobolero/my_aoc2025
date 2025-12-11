use std::collections::{HashMap, HashSet};
const INPUT: &str = include_str!("../../inputs/input11.txt");

// node of acyclic directed graph
// we don't use "pointers/references" but indexes into vector of nodes to
// satisfy Rust's borrow checker
// #[derive(Debug)]
struct Node {
    // name: &'a str,
    // index: usize,
    ways_out: u64,
    // visited: bool,
    referenced_nodes: Vec<usize>,
    referencing_nodes: Vec<usize>,
}

pub fn part1() -> u64 {
    let mut name_2_index: HashMap<&str, usize> = HashMap::new();
    let mut nodes: Vec<Node> = Vec::with_capacity(1024);
    let mut count = 0usize;
    for line in INPUT.lines() {
        let mut parts = line.split(':');
        let name = parts.next().unwrap();
        let descendents = parts.next().unwrap().split_ascii_whitespace();
        let current_index: usize;
        if name_2_index.contains_key(name) {
            current_index = *name_2_index.get(name).unwrap();
        } else {
            nodes.push(Node {
                // name,
                // index: count,
                ways_out: 0,
                // visited: false,
                referenced_nodes: Vec::new(),
                referencing_nodes: Vec::new(),
            });
            count += 1;
            current_index = count - 1;
            name_2_index.insert(name, count - 1);
        }
        for desc in descendents {
            if name_2_index.contains_key(desc) {
                let referenced = *name_2_index.get(desc).unwrap();
                nodes[current_index].referenced_nodes.push(referenced);
                nodes[referenced].referencing_nodes.push(current_index);
            } else {
                nodes.push(Node {
                    // name: desc,
                    // index: count,
                    ways_out: 0,
                    // visited: false,
                    referenced_nodes: Vec::new(),
                    referencing_nodes: Vec::new(),
                });
                count += 1;
                name_2_index.insert(desc, count - 1);
                nodes[current_index].referenced_nodes.push(count - 1);
                nodes[count - 1].referencing_nodes.push(current_index);
            }
        }
    }
    // println!("nodes: {:?}", nodes);
    let out = *name_2_index.get("out").unwrap();
    nodes[out].ways_out = 1;
    // nodes[out].visited = true;
    let you = *name_2_index.get("you").unwrap();
    let mut node_set: HashSet<usize> = HashSet::new();
    node_set.insert(out);

    loop {
        let mut next_nodes: HashSet<usize> = HashSet::with_capacity(1024);
        for node in &node_set {
            let ways_out = nodes[*node].ways_out;
            // find other nodes referencing the node
            for parent in nodes[*node].referencing_nodes.clone() {
                nodes[parent].ways_out += ways_out;
                // nodes[parent].visited = true;
                // println!("{:?}", nodes[parent]);
            }
            if you != *node {
                nodes[*node].ways_out = 0;
            }
            next_nodes.extend(nodes[*node].referencing_nodes.iter());
        }
        if next_nodes.is_empty() {
            break;
        }
        node_set = next_nodes;
        // println!();
    }
    nodes[you].ways_out
}

pub fn part2() -> u64 {
    let mut name_2_index: HashMap<&str, usize> = HashMap::new();
    let mut nodes: Vec<Node> = Vec::with_capacity(1024);
    let mut count = 0usize;
    for line in INPUT.lines() {
        let mut parts = line.split(':');
        let name = parts.next().unwrap();
        let descendents = parts.next().unwrap().split_ascii_whitespace();
        let current_index: usize;
        if name_2_index.contains_key(name) {
            current_index = *name_2_index.get(name).unwrap();
        } else {
            nodes.push(Node {
                // name,
                // index: count,
                ways_out: 0,
                // visited: false,
                referenced_nodes: Vec::new(),
                referencing_nodes: Vec::new(),
            });
            count += 1;
            current_index = count - 1;
            name_2_index.insert(name, count - 1);
        }
        for desc in descendents {
            if name_2_index.contains_key(desc) {
                let referenced = *name_2_index.get(desc).unwrap();
                nodes[current_index].referenced_nodes.push(referenced);
                nodes[referenced].referencing_nodes.push(current_index);
            } else {
                nodes.push(Node {
                    // name: desc,
                    // index: count,
                    ways_out: 0,
                    // visited: false,
                    referenced_nodes: Vec::new(),
                    referencing_nodes: Vec::new(),
                });
                count += 1;
                name_2_index.insert(desc, count - 1);
                nodes[current_index].referenced_nodes.push(count - 1);
                nodes[count - 1].referencing_nodes.push(current_index);
            }
        }
    }
    // svr -> fft -> dac -> out

    // phase 1
    // dac -> out
    let out = *name_2_index.get("out").unwrap();
    nodes[out].ways_out = 1;
    // nodes[out].visited = true;
    let you = *name_2_index.get("dac").unwrap();
    let mut node_set: HashSet<usize> = HashSet::new();
    node_set.insert(out);

    loop {
        let mut next_nodes: HashSet<usize> = HashSet::with_capacity(1024);
        for node in &node_set {
            let ways_out = nodes[*node].ways_out;
            // find other nodes referencing the node
            for parent in nodes[*node].referencing_nodes.clone() {
                nodes[parent].ways_out += ways_out;
                // nodes[parent].visited = true;
                // println!("{:?}", nodes[parent]);
            }
            if you != *node {
                nodes[*node].ways_out = 0;
            }
            next_nodes.extend(nodes[*node].referencing_nodes.iter());
        }
        if next_nodes.is_empty() {
            break;
        }
        node_set = next_nodes;
        // println!();
    }
    // phase 2
    // fft -> dac
    let out = *name_2_index.get("dac").unwrap();
    let you = *name_2_index.get("fft").unwrap();
    let mut node_set: HashSet<usize> = HashSet::new();
    node_set.insert(out);
    loop {
        let mut next_nodes: HashSet<usize> = HashSet::with_capacity(1024);
        for node in &node_set {
            let ways_out = nodes[*node].ways_out;
            // find other nodes referencing the node
            for parent in nodes[*node].referencing_nodes.clone() {
                nodes[parent].ways_out += ways_out;
                // nodes[parent].visited = true;
                // println!("{:?}", nodes[parent]);
            }
            if you != *node {
                nodes[*node].ways_out = 0;
            }
            next_nodes.extend(nodes[*node].referencing_nodes.iter());
        }
        if next_nodes.is_empty() {
            break;
        }
        node_set = next_nodes;
        // println!();
    }
    // phase 3
    // you -> fft
    let out = *name_2_index.get("fft").unwrap();
    let you = *name_2_index.get("svr").unwrap();
    let mut node_set: HashSet<usize> = HashSet::new();
    node_set.insert(out);
    loop {
        let mut next_nodes: HashSet<usize> = HashSet::with_capacity(1024);
        for node in &node_set {
            let ways_out = nodes[*node].ways_out;
            // find other nodes referencing the node
            for parent in nodes[*node].referencing_nodes.clone() {
                nodes[parent].ways_out += ways_out;
                // nodes[parent].visited = true;
                // println!("{:?}", nodes[parent]);
            }
            if you != *node {
                nodes[*node].ways_out = 0;
            }
            next_nodes.extend(nodes[*node].referencing_nodes.iter());
        }
        if next_nodes.is_empty() {
            break;
        }
        node_set = next_nodes;
        // println!();
    }
    nodes[you].ways_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 385912350172800);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 0);
    }
}
