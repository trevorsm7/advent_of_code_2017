use std::{fs, env};
use std::io::Error;

struct Machine {

}

pub fn day25(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day25.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn part1(input: &str) -> usize {
    let machine = parse_machine(input);

    0
}

fn parse_machine(input: &str) -> Machine {
    Machine {}
}
