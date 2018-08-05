use std::env;
use std::io;
use std::collections::VecDeque;

fn part1(step: usize, cycles: usize) -> usize {
    // We're going to add 1 to step every time...
    let step = step + 1;

    let mut deque = VecDeque::with_capacity(cycles + 1);

    // For each cycle, insert then update position
    let mut position = 0;
    for i in 0..cycles {
        deque.insert(position, i);
        position = (position + step) % deque.len();
    }

    // Return the element where we would have inserted
    deque[position]
}

#[test]
fn test_day17_part1() {
    assert_eq!(part1(3, 2017), 638);
}

fn part2(step: usize, cycles: usize) -> usize {
    // We're going to add 1 to step every time...
    let step = step + 1;

    let mut value = 0;

    // For each cycle, check if we're inserting after 0
    let mut position = 0;
    for i in 1..=cycles {
        position = (position + step) % i;
        if position == 0 {
            value = i;
        }
    }

    value
}

#[test]
fn test_day17_part2() {
    assert_eq!(part2(3, 9), 9);
}

pub fn day17(args: &mut env::Args) -> Result<(), io::Error> {
    // Read input from command line
    let input = args.next().unwrap_or("386".to_string()).parse().unwrap();

    println!("Part 1: {}", part1(input, 2017));
    println!("Part 2: {}", part2(input, 50_000_000));

    Ok(())
}
