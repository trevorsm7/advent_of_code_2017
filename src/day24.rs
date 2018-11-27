use std::usize;
use std::{fs, env};
use std::io::Error;
use std::collections::HashMap;

type Port = u32;
type Component = [Port; 2];

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
    let components = read_components(input);

    // Build a lookup map from pins to component index
    let lookup = build_lookup(&components);

    let mut max_score = 0;

    let mut visited_indices: Vec<usize> = Vec::new();
    let mut visited_outputs: Vec<Port> = Vec::new();

    let mut pending_indices: Vec<usize> = Vec::new();
    let mut pending_parents: Vec<usize> = Vec::new();
    let mut pending_inputs: Vec<Port> = Vec::new();

    let mut next_parent = usize::MAX;
    let mut next_output = 0;
    loop {
        // Find all components matching the input
        let mut found = false;
        if let Some(components) = lookup.get(&next_output) {
            for index in components.iter().cloned() {
                if visited_indices.contains(&index) { continue }

                // Add them on top of the pending stack
                pending_indices.push(index);
                pending_parents.push(next_parent);
                pending_inputs.push(next_output);
                found = true;
            }
        }

        if !found {
            // Tally-up the strength score before unwinding
            if let Some(&last) = visited_outputs.last() {
                let sum = visited_outputs.iter().cloned().sum::<Port>();
                // Score of (0,1)(1,2) is (0 + 1) + (1 + 2) -> (1 + 2) * 2 - 2
                let score = sum * 2 - last;
                if score > max_score { max_score = score }
                //println!("Score {}", score);
            }
        }

        // If any components left, get index and parent
        if pending_indices.is_empty() { break }
        let index = pending_indices.pop().unwrap();
        let parent = pending_parents.pop().unwrap();
        let input = pending_inputs.pop().unwrap();

        // Find the position of parent on the visited stack
        if let Some(pos) = visited_indices.iter().cloned().position(|x| x == parent) {
            // Clear visited stack after parent
            //println!("Push ({:?}) at {}", components[index], pos + 1);
            visited_indices.resize(pos + 1, 0);
            visited_outputs.resize(pos + 1, 0);
        }
        else {
            // Clear visited stack up to root
            //println!("Push ({:?}) at 0", components[index]);
            visited_indices.clear();
            visited_outputs.clear();
        }

        next_parent = index;
        let comp = components[index];
        next_output = if input == comp[0] { comp[1] } else { comp[0] };
        visited_indices.push(next_parent);
        visited_outputs.push(next_output);
    }

    max_score
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

fn build_lookup(components: &Vec<Component>) -> HashMap<Port, Vec<usize>> {
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
    input.trim().lines()
        .map(|line| {
            let mut it = line.split('/');
            let a = it.next().unwrap().parse().unwrap();
            let b = it.next().unwrap().parse().unwrap();
            [a, b]
        })
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
