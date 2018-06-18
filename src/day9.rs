use std::fs;
use std::env;
use std::io;

fn part1(input: &str) -> u32 {
    // Mutable state for filter_map below
    let mut escape = false;
    let mut garbage = false;

    // Feed each char through a state machine producing the score
    let (_, score) = input
        .chars()
        // A state machine for discarding garbage
        .filter_map(|c| {
            let (escape_next, garbage_next, c_next) = match (escape, garbage, c) {
                (true, a, _) => (false, a, None),
                (_, true, '!') => (true, true, None),
                (a, true, '>') => (a, false, None),
                (a, true, _) => (a, true, None),
                (a, _, '<') => (a, true, None),
                (a, b, c) => (a, b, Some(c)),
            };
            escape = escape_next;
            garbage = garbage_next;
            c_next
        })
        // A (much simpler) state machine for scoring groups
        .fold((0, 0), |(depth, score), c| match c {
            '{' => (depth + 1, score + depth + 1),
            '}' => (depth - 1, score),
            _ => (depth, score),
        });

    score
}

#[test]
fn test_day9() {
    // Test garbage
    assert_eq!(part1("<>"), 0);
    assert_eq!(part1("<random characters>"), 0);
    assert_eq!(part1("<<<<>"), 0);
    assert_eq!(part1("<{!>}>"), 0);
    assert_eq!(part1("<!!>"), 0);
    assert_eq!(part1("<!!!>>"), 0);
    assert_eq!(part1("<{o\"i!a,<{i<a>"), 0);

    // Test groups
    assert_eq!(part1("{}"), 1);
    assert_eq!(part1("{{{}}}"), 6);
    assert_eq!(part1("{{},{}}"), 5);
    assert_eq!(part1("{{{},{},{{}}}}"), 16);
    assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}

pub fn day9(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day9.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
