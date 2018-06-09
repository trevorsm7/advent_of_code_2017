use std::fs;
use std::cmp;
use std::env;

fn part1(input: &str) -> u32 {
    // Sum the differences over each line
    input.lines().fold(0, |sum, line| {
        // Parse the line into u32 tokens
        let numbers = line
            .split_whitespace()
            .map(|tok| tok.parse::<u32>().expect("expected a number"));

        // Accumulate min and max over the line
        let (min, max) = {
            let max_min = (std::u32::MAX, std::u32::MIN);
            numbers.fold(max_min, |acc, n| (cmp::min(acc.0, n), cmp::max(acc.1, n)))
        };

        // Add the difference to the sum
        if max >= min {sum + max - min} else {sum}
    })
}

fn part2(input: &str) -> u32 {
    // Sum the divisions over each line
    input.lines().fold(0, |sum, line| {
        // Parse the line into u32 tokens
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|tok| tok.parse::<u32>().expect("expected a number"))
            .collect();

        // Iterate over each pair of numbers
        for (i, a) in numbers.iter().enumerate() {
            for b in numbers.iter().skip(i + 1) {
                // If we match a pair, add it to the sum
                return sum + match (a, b) {
                    (a, b) if a % b == 0 => (a / b) as u32,
                    (a, b) if b % a == 0 => (b / a) as u32,
                    _ => continue
                }
            }
        }

        // If we never found a match, just return the sum
        println!("Warning: failed to match line {}", line);
        sum
    })
}

fn main() -> Result<(), std::io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = env::args().skip(1).next().unwrap_or("day2.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
