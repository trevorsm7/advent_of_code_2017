use std::fs;
use std::env;
use std::io;

// Run a single iteration of the hash function
fn do_hash(size: usize, lengths: &[u8], list: &mut [usize], position_skip: &mut (usize, usize)) {
    let (ref mut position, ref mut skip) = position_skip;

    for length in lengths {
        let length = *length as usize;
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
fn sparse_hash(size: usize, lengths: &[u8]) -> Vec<usize> {
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

// Compute the dense hash for a given input
fn dense_hash(size: usize, lengths: &[u8]) -> Vec<usize> {
    // Compute the sparse hash first
    let sparse = sparse_hash(size, lengths);

    // Mutable state for filter_map
    let mut hash = 0;
    let mut count = 0;

    // Reduce and collect to dense hash
    sparse
        .iter()
        .filter_map(|i| {
            // Reduce by XORing over blocks of 16, yielding 1 out per 16 in
            hash = if count == 0 { *i } else { hash ^ *i };
            count = (count + 1) % 16;
            if count == 0 { Some(hash) } else { None }
        })
        .collect()
}

fn part1(size: usize, input: &str) -> usize {
    assert!(size >= 2);

    // For each length in the input...
    let lengths: Vec<u8> = input
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
fn test_day10_part1() {
    assert_eq!(part1(5, "3,4,1,5"), 12);
}

fn part2(input: &str) -> String {
    // We must discard the EOL whitespace
    let input = input.trim_right();

    // We want ASCII, but UTF-8 will have to do
    assert_eq!(input.len(), input.chars().count(), "Multi-byte UTF-8 chars are not allowed");
    let mut lengths = input.to_string().into_bytes();

    // Append the given magic numbers
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    // Compute the dense hash (given size 256)
    let dense = dense_hash(256, &lengths);

    // Convert dense hash to hex string
    let mut hex = String::with_capacity(32);
    for i in &dense {
        hex.push_str(&format!("{:02x}", *i as u8));
    }
    hex
}

#[test]
fn test_day10_part2() {
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

pub fn day10(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day10.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(256, &input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
