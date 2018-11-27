use std::{fs, env};
use std::io::Error;
use std::collections::HashMap;

type Port = u16;
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

fn part1(input: &str) -> u32 {
    let components = read_components(input);

    // Build a lookup map from pins to component index
    let lookup = build_lookup(&components);

    let mut strength = 0;
    let mut max_strength = 0;

    /*let mut visited_indices = Vec::new();
    let mut visited_outputs = Vec::new();

    let mut pending_indices = Vec::new();
    let mut pending_parents = Vec::new();*/



    max_strength
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
