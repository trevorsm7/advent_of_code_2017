use std::fs;
use std::env;
use std::io;

fn part1(tokens: &str) -> u32 {
    // Parse the line into u32 tokens
    let mut jumps: Vec<i32> = tokens
        .split_whitespace()
        .map(|tok| tok.parse::<i32>().expect("expected a number"))
        .collect();

    let mut index = 0;
    let mut count = 0;

    // Loop until index leaves table
    while index >= 0 && index < jumps.len() as i32 {
        // Compute the next index and increment the old one
        let new_index = index + jumps[index as usize];
        jumps[index as usize] += 1;

        // Jump and increment the jump count
        index = new_index;
        count = count + 1;
    }

    count
}

#[test]
fn test_day5_part1() {
    assert_eq!(part1("0 3 0 1 -3"), 5);
}

pub fn day5(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day5.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}
