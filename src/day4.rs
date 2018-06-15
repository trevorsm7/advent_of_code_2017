use std::fs;
use std::env;
use std::io;
use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    // Sum the number of valid lines
    input.lines().fold(0, |sum, line| {
        // Parse the line into passphrase tokens
        let passphrases = line.split_whitespace();

        // Insert phrases into set, returning if we encounter a duplicate
        let mut unique_set = HashSet::new();
        for passphrase in passphrases {
            if !unique_set.insert(passphrase) {
                return sum;
            }
        }

        return sum + 1;
    })
}

#[test]
fn test_day4_part1() {
    assert_eq!(part1("aa bb cc dd ee"), 1);
    assert_eq!(part1("aa bb cc dd aa"), 0);
    assert_eq!(part1("aa bb cc dd aaa"), 1);
    assert_eq!(part1("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa"), 2);
}

pub fn day4(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day4.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}
