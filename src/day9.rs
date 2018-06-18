use std::fs;
use std::env;
use std::io;

fn dewit(input: &str) -> (u32, u32) {
    // Mutable state for filter_map below
    let mut escape = false;
    let mut garbage = false;
    let mut count = 0;

    // Feed each char through a state machine producing the score
    let (_, score) = input
        .chars()
        // A state machine for discarding garbage
        .filter_map(|c| match (escape, garbage, c) {
            (true, _, _) => { escape = false; None },
            (_, true, '!') => { escape = true; None },
            (_, true, '>') => { garbage = false; None },
            (_, true, _) => { count += 1; None },
            (_, _, '<') => { garbage = true; None },
            (_, _, c) => Some(c),
        })
        // A (much simpler) state machine for scoring groups
        .fold((0, 0), |(depth, score), c| match c {
            '{' => (depth + 1, score + depth + 1),
            '}' => (depth - 1, score),
            _ => (depth, score),
        });

    (score, count)
}

#[test]
fn test_day9() {
    // Test garbage
    assert_eq!(dewit("<>"), (0, 0));
    assert_eq!(dewit("<random characters>"), (0, 17));
    assert_eq!(dewit("<<<<>"), (0, 3));
    assert_eq!(dewit("<{!>}>"), (0, 2));
    assert_eq!(dewit("<!!>"), (0, 0));
    assert_eq!(dewit("<!!!>>"), (0, 0));
    assert_eq!(dewit("<{o\"i!a,<{i<a>"), (0, 10));

    // Test groups
    assert_eq!(dewit("{}"), (1, 0));
    assert_eq!(dewit("{{{}}}"), (6, 0));
    assert_eq!(dewit("{{},{}}"), (5, 0));
    assert_eq!(dewit("{{{},{},{{}}}}"), (16, 0));
    assert_eq!(dewit("{<a>,<a>,<a>,<a>}"), (1, 4));
    assert_eq!(dewit("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
    assert_eq!(dewit("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
    assert_eq!(dewit("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
}

pub fn day9(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day9.txt".to_string());
        fs::read_to_string(name)?
    };

    let (part1, part2) = dewit(&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
