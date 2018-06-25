use std::fs;
use std::env;
use std::io;

use std::collections::{BTreeMap, BTreeSet};
type Graph = BTreeMap<usize, Vec<usize>>;
type Reachable = BTreeSet<usize>;

// Build the graph of pipes
fn build_graph(input: &str) -> Graph {
    let mut graph = Graph::new();

    // For each line of input...
    for line in input.lines() {
        // Filter whitepsace and punctuation
        let mut iter = line
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|s| !s.is_empty());

        // Parse PID and its neighbors
        let pid = iter.next().unwrap().parse().unwrap();
        let pipes: Vec<usize> = iter.map(|tok| tok.parse().unwrap()).collect();
        graph.insert(pid, pipes);
    }

    graph
}

// Build the set of reachable programs
fn build_reachable(graph: &Graph, from: usize) -> Reachable {
    let mut reachable = Reachable::new();

    // Do DFS starting at node `from`
    let mut to_visit = vec![from];
    while !to_visit.is_empty() {
        // Visit the node on top of the stack
        let pid = to_visit.pop().unwrap();
        if reachable.insert(pid) {
            // Push reachable nodes on top of stack
            let v = graph.get(&pid).unwrap();
            to_visit.extend_from_slice(&v);
        }
    }

    reachable
}

fn part1(input: &str) -> usize {
    let graph = build_graph(input);
    let reachable = build_reachable(&graph, 0);
    reachable.len()
}

#[test]
fn test_day12() {
    let input =
        "0 <-> 2
        1 <-> 1
        2 <-> 0, 3, 4
        3 <-> 2, 4
        4 <-> 2, 3, 6
        5 <-> 6
        6 <-> 4, 5";
    assert_eq!(part1(&input), 6);
}

pub fn day12(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day12.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
