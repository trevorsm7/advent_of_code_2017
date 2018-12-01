use std::{fs, env};
use std::io::Error;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Branch {
    next: usize,
    value: bool,
    right: bool,
}

type State = [Branch; 2];

struct Machine {
    state: usize,
    steps: usize,
    states: Vec<State>,
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
    let mut machine = parse_machine(input);
    let mut tape: HashSet<isize> = HashSet::new();
    let mut pos = 0;

    for _ in 0..machine.steps {
        let value = if tape.contains(&pos) { 1 } else { 0 };
        let branch = &machine.states[machine.state][value];
        if branch.value { tape.insert(pos); } else { tape.remove(&pos); }
        if branch.right { pos += 1 } else { pos -= 1 }
        machine.state = branch.next;
    }

    tape.len()
}

#[test]
fn test_part1() {
    let input =
"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
    assert_eq!(part1(&input), 3);
}

fn parse_machine(input: &str) -> Machine {
    let mut lines = input.trim().lines();

    // Parse initial state from first line
    let header = "Begin in state ";
    let line = lines.next().unwrap();
    assert_eq!(line.starts_with(line), true);
    let token = &line[header.len()..].trim_matches('.');
    let state = usize::from_str_radix(token, 36).unwrap() - 10;

    // Parse steps until diagnostic checksum
    let header = "Perform a diagnostic checksum after ";
    let line = lines.next().unwrap();
    assert_eq!(line.starts_with(line), true);
    let token = &line[header.len()..].split_whitespace().next().unwrap();
    let steps = token.parse().unwrap();

    let mut states = Vec::new();

    while let Some(_) = lines.next() {
        let header = "In state ";
        let line = lines.next().unwrap();
        assert_eq!(line.starts_with(line), true);
        let token = &line[header.len()..].trim_matches(':');
        let state = usize::from_str_radix(token, 36).unwrap() - 10;
        assert_eq!(state, states.len());

        let mut e = [Branch::new(); 2];

        for i in 0..2 {
            let header = "  If the current value is ";
            let line = lines.next().unwrap();
            assert_eq!(line.starts_with(line), true);
            let token = &line[header.len()..].trim_matches(':');
            let value: usize = token.parse().unwrap();
            assert_eq!(value, i);

            let header = "    - Write the value ";
            let line = lines.next().unwrap();
            assert_eq!(line.starts_with(line), true);
            e[i].value = match &line[header.len()..] {
                "1." => true,
                "0." => false,
                _ => panic!("Expected '0.' or '1.'")
            };

            let header = "    - Move one slot to the ";
            let line = lines.next().unwrap();
            assert_eq!(line.starts_with(line), true);
            e[i].right = match &line[header.len()..] {
                "right." => true,
                "left." => false,
                _ => panic!("Expected 'left.' or 'right.'")
            };

            let header = "    - Continue with state ";
            let line = lines.next().unwrap();
            assert_eq!(line.starts_with(line), true);
            let token = &line[header.len()..].trim_matches('.');
            e[i].next = usize::from_str_radix(token, 36).unwrap() - 10;
        }

        states.push(e);
    }

    Machine { state, steps, states }
}

#[test]
fn test_day25_parse_machine() {
    let input =
"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
    let machine = parse_machine(&input);
    assert_eq!(machine.state, 0);
    assert_eq!(machine.steps, 6);
    assert_eq!(&machine.states,
        &[[Branch { next: 1, value: true, right: true },
           Branch { next: 1, value: false, right: false }],
          [Branch { next: 0, value: true, right: false },
           Branch { next: 0, value: true, right: true }]]);
}

impl Branch {
    fn new() -> Self {
        Self { next: 0, value: false, right: false }
    }
}
