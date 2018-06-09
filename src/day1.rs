use std::iter::Iterator;
use std::env::Args;
use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::iter::Peekable;

fn peek_or<T>(iter: &mut Peekable<T>, alt: u32) -> u32
    where T: Iterator<Item=char> {
    iter.peek().map_or(Some(alt), |ch| ch.to_digit(10)).unwrap_or(alt)
}

fn part1_peek(input: &str) -> u32 {
    let mut iter = input.chars().peekable();

    // Get the first digit (for the wraparound)
    let first = peek_or(&mut iter, 0);

    // Sum while looping over each character
    let mut sum = 0;
    loop {
        match iter.next() {
            Some(ch) => {
                // Parse the current digit
                let digit = match ch.to_digit(10) {
                    Some(digit) => digit,
                    // Bail if we parse a non-digit
                    None => break
                };

                // Sum digit if it's the same as the next (or the first)
                if digit == peek_or(&mut iter, first) {
                    sum += digit;
                }
            },
            // Bail if we run out of chars
            None => break
        }
    }

    sum
}

fn part1_zip(input: &str) -> u32 {
    // Get two iterators, one of which is offset by 1 and has wraparound
    let iter1 = input.chars().map(|ch| ch.to_digit(10).expect("expected a digit"));
    let iter2 = input.chars().cycle().skip(1).map(|ch| ch.to_digit(10).expect("expected a digit"));

    // Sum each digit that matches the following digit
    iter1.zip(iter2).fold(0, |sum, (digit, next)| if digit == next {sum + digit} else {sum})
}

fn part1_scan(input: &str) -> u32 {
    // We use a single iterator with wraparound
    let mut iter = input.chars().cycle().map(|ch| ch.to_digit(10).expect("expected a digit"));

    let len = input.chars().count();
    let first = iter.next().unwrap_or(0);

    // Use scan to remember the previous digit and return if matching
    // In this way, we make sure that each digit is parsed only once
    // Finally, we use fold to sum all the results
    iter.scan(first, |prev, digit| {
        let res = if digit == *prev {digit} else {0};
        *prev = digit;
        Some(res)
    }).take(len).fold(0, |sum, digit| sum + digit)
}

#[cfg(test)]
fn test_part1(f: &Fn(&str) -> u32) {
    assert_eq!(f("1122"), 3);
    assert_eq!(f("1111"), 4);
    assert_eq!(f("1234"), 0);
    assert_eq!(f("91212129"), 9);
}

#[test]
fn test_part1_peek() {
    test_part1(&part1_peek);
}

#[test]
fn test_part1_zip() {
    test_part1(&part1_zip);
}

#[test]
fn test_part1_scan() {
    test_part1(&part1_scan);
}

fn part2_vec(input: &str) -> u32 {
    // Get digits as a vector
    let digits: Vec<u32> = input.chars().map(|ch| ch.to_digit(10).expect("expected a digit")).collect();
    let half_len = digits.len() / 2;

    // Iterate over vector summing each matched digit
    let mut sum = 0;
    for i in 0..digits.len() {
        let j = (i + half_len) % digits.len();
        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    sum
}

fn part2_zip(input: &str) -> u32 {
    let half_len = input.chars().count() / 2;

    // Get two iterators, one of which is offset by half_len and has wraparound
    let iter1 = input.chars().map(|ch| ch.to_digit(10).expect("expected a digit"));
    let iter2 = input.chars().cycle().skip(half_len).map(|ch| ch.to_digit(10).expect("expected a digit"));

    // Sum each digit that matches the following digit
    iter1.zip(iter2).fold(0, |sum, (digit, next)| if digit == next {sum + digit} else {sum})
}

#[cfg(test)]
fn test_part2(f: &Fn(&str) -> u32) {
    assert_eq!(f("1212"), 6);
    assert_eq!(f("1221"), 0);
    assert_eq!(f("123425"), 4);
    assert_eq!(f("123123"), 12);
    assert_eq!(f("12131415"), 4);
}

#[test]
fn test_part2_vec() {
    test_part2(&part2_vec);
}

#[test]
fn test_part2_zip() {
    test_part2(&part2_zip);
}

pub fn day1(args: &mut Args) -> Result<(), Error> {
    // Open file in first arg or default to input.txt
    let mut file = {
        let name = args.next().unwrap_or("input/day1.txt".to_string());
        File::open(Path::new(&name))?
    };

    // File length is u64 but with_capacity takes usize!
    let mut input = {
        let len = file.metadata().unwrap().len();
        String::with_capacity(len as usize)
    };

    // Read file into string, truncating whitespace
    file.read_to_string(&mut input)?;
    let len = input.trim_right().len();
    input.truncate(len);

    println!("Part 1 (peek): {}", part1_peek(&input));
    println!("Part 1 (zip): {}", part1_zip(&input));
    println!("Part 1 (scan): {}", part1_scan(&input));
    println!("Part 2 (vec): {}", part2_vec(&input));
    println!("Part 2 (zip): {}", part2_zip(&input));

    Ok(())
}
