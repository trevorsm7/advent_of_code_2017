use std::fs;
use std::cmp;
use std::env;
use std::io;
use std::u32;

fn part1(input: &str) -> u32 {
    // Sum the differences over each line
    input.lines().fold(0, |sum, line| {
        // Parse the line into u32 tokens
        let numbers = line
            .split_whitespace()
            .map(|tok| tok.parse::<u32>().expect("expected a number"));

        // Accumulate min and max over the line
        let (min, max) = {
            let max_min = (u32::MAX, u32::MIN);
            numbers.fold(max_min, |acc, n| (cmp::min(acc.0, n), cmp::max(acc.1, n)))
        };

        // Add the difference to the sum
        if max >= min {sum + max - min} else {sum}
    })
}

#[test]
fn test_part1() {
    assert_eq!(part1("5 1 9 5"), 8);
    assert_eq!(part1("7 5 3"), 4);
    assert_eq!(part1("2 4 6 8"), 6);
    assert_eq!(part1("5 1 9 5\n7 5 3\n2 4 6 8"), 18)
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

#[test]
fn test_part2() {
    assert_eq!(part2("5 9 2 8"), 4);
    assert_eq!(part2("9 4 7 3"), 3);
    assert_eq!(part2("3 8 6 5"), 2);
    assert_eq!(part2("5 9 2 8\n9 4 7 3\n3 8 6 5"), 9)
}

pub fn day2(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day2.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
