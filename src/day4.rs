use std::fs;
use std::env;
use std::io;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

fn part1(input: &str) -> u32 {
    // Sum the number of valid lines
    input.lines().fold(0, |sum, line| {
        // Parse the line into passphrase tokens
        let passphrases = line.split_whitespace();

        // Insert phrases into set one at a time
        let mut unique_set = BTreeSet::new();
        for passphrase in passphrases {
            // Passphrase is invalid if we encounter a duplicate
            if !unique_set.insert(passphrase) {
                return sum;
            }
        }

        // Passphrase is valid
        sum + 1
    })
}

#[test]
fn test_day4_part1() {
    assert_eq!(part1("aa bb cc dd ee"), 1);
    assert_eq!(part1("aa bb cc dd aa"), 0);
    assert_eq!(part1("aa bb cc dd aaa"), 1);
    assert_eq!(part1("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa"), 2);
}

fn count_chars(token: &str) -> BTreeMap<char, u32> {
    let mut counts = BTreeMap::new();

    // For each char in the token
    for c in token.chars() {
        // Increment the tally for this char
        let count = *counts.get(&c).unwrap_or(&0);
        counts.insert(c, count + 1);
    }

    counts
}

fn is_anagram(a: &str, b: &str) -> bool {
    // Anagrams must have equal length
    if a.len() != b.len() {
        return false;
    }

    // Tally the char counts in each token
    let a_counts = count_chars(a);
    let b_counts = count_chars(b);

    // For each unique char in a
    for c in a_counts.keys() {
        // Get the number of occurences of the char
        let a_count = a_counts.get(&c).unwrap_or(&0);
        let b_count = b_counts.get(&c).unwrap_or(&0);

        // Anagrams must have the same tally for each char
        if a_count != b_count {
            return false;
        }
    }

    true
}

fn part2(input: &str) -> u32 {
    // Sum the number of valid lines
    input.lines().fold(0, |sum, line| {
        // Parse the line into passphrase tokens
        let passphrases: Vec<&str> = line.split_whitespace().collect();

        // Loop over each permutation of pairs
        for (i, a) in passphrases.iter().enumerate() {
            for j in i+1..passphrases.len() {
                // Passphrase is invalid if we find an anagram
                let b = &passphrases[j];
                if is_anagram(a, b) {
                    return sum;
                }
            }
        }

        // Passphrase is valid
        sum + 1
    })
}

#[test]
fn test_day4_part2() {
    assert_eq!(part2("abcde fghij"), 1);
    assert_eq!(part2("abcde xyz ecdab"), 0);
    assert_eq!(part2("a ab abc abd abf abj"), 1);
    assert_eq!(part2("iiii oiii ooii oooi oooo"), 1);
    assert_eq!(part2("oiii ioii iioi iiio"), 0);
    assert_eq!(part2("abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio"), 3);
}

pub fn day4(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day4.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
