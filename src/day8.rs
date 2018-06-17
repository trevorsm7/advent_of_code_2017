use std::fs;
use std::env;
use std::io;
use std::ops::{Add, Sub};
use std::cmp::max;
use std::collections::HashMap;

fn dewit(input: &str) -> (i32, i32) {
    let mut regs = HashMap::new();
    let mut max_value = 0;

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
            // Update the register value
            let mut value = *regs.get(name).unwrap_or(&0);
            value = op(value, amount);
            regs.insert(name, value);

            // Record the max value seen
            max_value = max(max_value, value);
        }
    }

    // Return the current largest value and the largest value seen
    (*regs.values().max().unwrap(), max_value)
}

#[test]
fn test_day8() {
    let input =
"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
    assert_eq!(dewit(&input), (1, 10));
}

pub fn day8(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day8.txt".to_string());
        fs::read_to_string(name)?
    };

    let (part1, part2) = dewit(&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
