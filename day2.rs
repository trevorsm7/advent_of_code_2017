use std::fs;
use std::cmp;
use std::env;

fn part1(input: &String) -> u32 {
    // Sum the differences over each line
    input.lines().fold(0, |sum, line| {
        // Parse the line into u32 tokens
        let numbers = line.split_whitespace().map(|tok| tok.parse::<u32>().expect("expected a number"));

        // Accumulate min and max over the line
        let (min, max) = {
            let max_min = (std::u32::MAX, std::u32::MIN);
            numbers.fold(max_min, |acc, n| (cmp::min(acc.0, n), cmp::max(acc.1, n)))
        };

        // Add the difference to the sum
        sum + max - min
    })
}

fn main() -> Result<(), std::io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = env::args().skip(1).next().unwrap_or("day2.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
