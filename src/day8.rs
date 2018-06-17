use std::fs;
use std::env;
use std::io;
use std::ops::{Add, Sub};
use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut regs = HashMap::new();

    // Parse the input line-by-line
    for line in input.lines() {
        // Split line with whitespace
        let mut iter = line.split_whitespace();

        // Parse the instruction
        let name = iter.next().unwrap();
        let op = match iter.next() {
            Some("inc") => i32::add,
            Some("dec") => i32::sub,
            _ => panic!("invalid operation"),
        };
        let amount: i32 = iter.next().unwrap().parse().unwrap();

        // Assume this token will always match "if"
        assert_eq!(iter.next().unwrap(), "if");

        // Parse the condition
        let left = *regs.get(iter.next().unwrap()).unwrap_or(&0);
        let cond = match iter.next() {
            Some("<") => i32::lt,
            Some("<=") => i32::le,
            Some("==") => i32::eq,
            Some("!=") => i32::ne,
            Some(">=") => i32::ge,
            Some(">") => i32::gt,
            _ => panic!("invalid condition"),
        };
        let right = iter.next().unwrap().parse().unwrap();

        // Evaluate the instruction
        if cond(&left, &right) {
            let old = *regs.get(name).unwrap_or(&0);
            regs.insert(name, op(old, amount));
        }
    }

    // Return the largest register value
    *regs.values().max().unwrap()
}

#[test]
fn test_day8_part1() {
    let input =
"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    assert_eq!(part1(&input), 1);
}

pub fn day8(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day8.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
