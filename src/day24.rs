use std::usize;
use std::{fs, env};
use std::io::Error;
use std::collections::HashMap;

type Port = u32;
type Component = [Port; 2];
type PortLookup = HashMap<Port, Vec<usize>>;

struct Visited {
    indices: Vec<usize>,
    outputs: Vec<Port>,
}

struct Pending {
    indices: Vec<usize>,
    parents: Vec<usize>,
    inputs: Vec<Port>,
}

struct Next {
    parent: usize,
    output: Port,
}

pub fn day24(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day24.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> Port {
    // Read input and build a lookup table
    let components = read_components(input);
    let lookup = build_lookup(&components);

    // Keep track of max score, visited item stack, pending item stack, and next item
    let mut max_score = 0;
    let mut visited = Visited::new();
    let mut pending = Pending::new();
    let mut next = Next::new();

    loop {
        // Find all components matching the input
        let found = find_component(&lookup, &mut visited, &mut pending, &mut next);

        // Tally-up the strength score whenever we run out of items
        if !found {
            update_score(&mut max_score, &mut visited);
        }

        // If we've tried everything, return the max score
        if pending.indices.is_empty() { return max_score }

        // Grab an item off the pending stack
        let (index, parent, input) = pending.pop();

        // Unwind the visited stack to the parent
        unwind_visited(&mut visited, parent);

        // Update the next state
        next.parent = index;
        let [port1, port2] = components[index];
        next.output = if input == port2 { port1 } else { port2 };

        // Add the item to the visited stack
        visited.push(next.parent, next.output);
    }
}

#[test]
fn test_day24_part1() {
    let input = "\
        0/2\n\
        2/2\n\
        2/3\n\
        3/4\n\
        3/5\n\
        0/1\n\
        10/1\n\
        9/10\n";
    assert_eq!(part1(&input), 31);
}

fn find_component(lookup: &PortLookup, visited: &mut Visited, pending: &mut Pending, next: &mut Next) -> bool {
    let mut found = false;

    if let Some(components) = lookup.get(&next.output) {
        for index in components.iter().cloned() {
            // Skip items already on the visited stack
            if visited.indices.contains(&index) { continue }

            // Add them on top of the pending stack
            pending.push(index, next.parent, next.output);
            found = true;
        }
    }

    found
}

fn update_score(max_score: &mut Port, visited: &mut Visited) {
    // Score of (0,1)(1,2) is (0 + 1) + (1 + 2) -> (1 + 2) * 2 - 2
    // So sum the outputs, multiply by 2, and subtract the last one
    if let Some(&last) = visited.outputs.last() {
        let sum = visited.outputs.iter().cloned().sum::<Port>();
        let score = sum * 2 - last;
        if score > *max_score { *max_score = score }
    }
}

fn unwind_visited(visited: &mut Visited, top: usize) {
    if let Some(pos) = visited.indices.iter().cloned().position(|x| x == top) {
        // Clear visited stack after top
        visited.indices.resize(pos + 1, 0);
        visited.outputs.resize(pos + 1, 0);
    }
    else {
        // Clear visited stack completely
        visited.indices.clear();
        visited.outputs.clear();
    }
}

fn build_lookup(components: &Vec<Component>) -> PortLookup {
    let mut lookup = HashMap::new();

    for i in 0..components.len() {
        for e in components[i].iter().cloned() {
            // Create an empty vec if not present
            if !lookup.contains_key(&e) {
                lookup.insert(e, Vec::new());
            }

            // Push the component index into the map
            let ref mut indices = lookup.get_mut(&e).unwrap();
            if !indices.contains(&i) {
                indices.push(i);
            }
        }
    }

    lookup
}

#[test]
fn test_day24_build_lookup() {
    let input = "\
        0/1\n\
        0/2\n\
        1/2\n";
    let components = read_components(&input);
    let lookup = build_lookup(&components);
    assert_eq!(lookup.get(&0), Some(&vec![0, 1]));
    assert_eq!(lookup.get(&1), Some(&vec![0, 2]));
    assert_eq!(lookup.get(&2), Some(&vec![1, 2]));
}

fn read_components(input: &str) -> Vec<Component> {
    // For each line
    input.trim().lines()
        .map(|line| {
            // Parse "a/b"
            let mut it = line.trim().split('/');
            let a = it.next().unwrap().parse().unwrap();
            let b = it.next().unwrap().parse().unwrap();
            [a, b]
        })
        // And collect the results
        .collect::<Vec<Component>>()
}

#[test]
fn test_day24_read_components() {
    let input = "\
        0/2\n\
        1/3\n\
        2/4\n";
    assert_eq!(&read_components(&input),
        &[[0, 2], [1, 3], [2, 4]]);
}

impl Visited {
    fn new() -> Self {
        let indices = Vec::new();
        let outputs = Vec::new();
        Self { indices, outputs }
    }

    fn push(&mut self, index: usize, output: Port) {
        self.indices.push(index);
        self.outputs.push(output);
    }

    /*fn pop(&mut self) -> (usize, Port) {
        let index = self.indices.pop().unwrap();
        let output = self.outputs.pop().unwrap();
        (index, output)
    }*/
}

impl Pending {
    fn new() -> Self {
        let indices = Vec::new();
        let parents = Vec::new();
        let inputs = Vec::new();
        Self { indices, parents, inputs }
    }

    fn push(&mut self, index: usize, parent: usize, input: Port) {
        self.indices.push(index);
        self.parents.push(parent);
        self.inputs.push(input);
    }

    fn pop(&mut self) -> (usize, usize, Port) {
        let index = self.indices.pop().unwrap();
        let parent = self.parents.pop().unwrap();
        let input = self.inputs.pop().unwrap();
        (index, parent, input)
    }
}

impl Next {
    fn new() -> Self {
        let parent = usize::MAX;
        let output = 0;
        Self { parent, output }
    }
}
