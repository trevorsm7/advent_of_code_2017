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
        let pipes = iter.map(|tok| tok.parse().unwrap()).collect();
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

fn dewit(input: &str) -> (usize, usize) {
    let graph = build_graph(input);

    // Count the nodes reachable from PID 0
    let mut reachable = build_reachable(&graph, 0);
    let first_len = reachable.len();

    let mut group_count = 0;

    // Remove each reachable set from the set of all keys, one at a time
    let mut remainder: Reachable = graph.keys().cloned().collect();
    loop {
        // Remove the reachable nodes and increment count
        remainder = remainder.difference(&reachable).cloned().collect();
        group_count += 1;

        if remainder.is_empty() {
            return (first_len, group_count);
        }

        // Get a PID, then find the next set of reachable nodes
        let pid = *remainder.iter().next().unwrap();
        reachable = build_reachable(&graph, pid);
    }
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
    assert_eq!(dewit(&input), (6, 2));
}

pub fn day12(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day12.txt".to_string());
        fs::read_to_string(name)?
    };

    let (part1, part2) = dewit(&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
