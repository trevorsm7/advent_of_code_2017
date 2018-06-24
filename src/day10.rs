use std::fs;
use std::env;
use std::io;

// Run a single iteration of the hash function
fn do_hash(size: usize, lengths: &[usize], list: &mut [usize], position_skip: &mut (usize, usize)) {
    let (ref mut position, ref mut skip) = position_skip;

    for &length in lengths {
        assert!(length <= size);

        // Reverse [length] elements
        // NOTE this works for length=1, but we can skip it
        // NOTE also, slices have a built-in reverse method! but we need to handle wrap-around
        if length > 1 {
            let mut i = *position;
            let mut j = (*position + length - 1) % size;
            while i != j {
                // Swap each end of the sublist
                let swap = list[i];
                list[i] = list[j];
                list[j] = swap;

                // Pop from each end of the sublist
                i = (i + 1) % size;
                if j == i {
                    // Stop when we swap the last two elements
                    break;
                }
                j = (size + j - 1) % size;
            }
        }

        // Update position and skip size
        *position = (*position + length + *skip) % size;
        *skip += 1;
    }
}

// Compute the sparse hash for a given input
fn sparse_hash(size: usize, lengths: &[usize]) -> Vec<usize> {
    assert!(size >= 2);

    // Initialize list with values incrementing from zero
    let mut list: Vec<usize> = (0..size).collect();

    // Run 64 iterations of the hash function
    let mut position_skip = (0, 0);
    for _ in 0..64 {
        do_hash(size, lengths, &mut list, &mut position_skip);
    }

    list
}

fn dense_hash(size: usize, lengths: &[usize]) {
    let list = sparse_hash(size, lengths);

    // TODO
}

fn part1(size: usize, input: &str) -> usize {
    assert!(size >= 2);

    // For each length in the input...
    let lengths: Vec<usize> = input
        .trim_right() // discard EOL whitespace
        .split(',')
        .map(|tok| tok.parse().expect("expected a number"))
        .collect();

    // Initialize list with values incrementing from zero
    let mut list: Vec<usize> = (0..size).collect();

    // Run the hash function once on the list
    do_hash(size, &lengths, &mut list, &mut (0, 0));

    list[0] * list[1]
}

#[test]
fn test_day10() {
    assert_eq!(part1(5, "3,4,1,5"), 12);
}

pub fn day10(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day10.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(256, &input));

    Ok(())
}
