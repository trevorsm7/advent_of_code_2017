use std::fs;
use std::env;
use std::io;
use std::collections::HashSet;

fn redistribute(banks: &mut Vec<usize>) {
    debug_assert!(!banks.is_empty());

    // Find bank with max blocks, favoring the first occurence
    let mut max = (0, banks[0]);
    for (i, v) in banks[1..].iter().enumerate() {
        if *v > max.1 {
            // We start from 1, not 0, so we must add 1 to i
            max = (i + 1, *v);
        }
    }

    // Remove blocks from max bank
    banks[max.0] = 0;
    let div = max.1 / banks.len();
    let rem = max.1 % banks.len();

    // Multiples of banks.len() are added to each element
    if div > 0 {
        for v in banks.iter_mut() {
            *v += div;
        }
    }

    // Remainder is redistributed starting from the next bank
    let start = max.0 + 1;
    for i in 0..rem {
        let index = (start + i) % banks.len();
        banks[index] += 1;
    }
}

fn count_cycles(banks: &mut Vec<usize>) -> usize {
    // Handle trivial case
    if banks.is_empty() {
        return 0;
    }

    // Create a set of bank configurations
    let mut seen_configs = HashSet::new();

    let mut cycles = 0;
    while seen_configs.insert(banks.clone()) {
        cycles += 1;
        redistribute(banks);
    }

    cycles
}

fn part1(banks: &Vec<usize>) -> usize {
    let mut copy = banks.clone();
    count_cycles(&mut copy)
}

#[test]
fn test_day6_part1() {
    assert_eq!(part1(&vec![0, 2, 7, 0]), 5);
}

fn part2(banks: &Vec<usize>) -> usize {
    // Surprise, we just call part1 twice
    let mut copy = banks.clone();
    count_cycles(&mut copy);
    count_cycles(&mut copy)
}

#[test]
fn test_day6_part2() {
    assert_eq!(part2(&vec![0, 2, 7, 0]), 4);
}

pub fn day6(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day6.txt".to_string());
        fs::read_to_string(name)?
    };

    // Parse each usize token as a memory bank
    let banks: Vec<usize> = input
        .split_whitespace()
        .map(|tok| tok.parse().expect("expected a number"))
        .collect();

    println!("Part 1: {}", part1(&banks));
    println!("Part 2: {}", part2(&banks));

    Ok(())
}
