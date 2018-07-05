use std::fs;
use std::env;
use std::io;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
struct Scanner {
    range: usize,
    offset: usize,
}

impl Scanner {
    fn with_range(range: usize) -> Self {
        Self { range, offset: 0 }
    }

    fn update(&mut self, steps: usize) {
        // Combine direction and layer into a single variable
        // [0, range-1) is forward, [range-1, 2*(range-1)) is reverse
        // This makes it trivial to take a large number of steps in a single update
        self.offset += steps;
        self.offset %= (self.range - 1) * 2;
    }

    #[allow(dead_code)]
    fn position(&self) -> usize {
        if self.offset >= self.range {
            // Map the reverse range to the actual position
            (self.range - 1) * 2 - self.offset
        }
        else { self.offset }
    }

    fn at_top(&self) -> bool {
        self.offset == 0
    }
}

type Firewall = BTreeMap<usize, Scanner>;

trait FirewallExt {
    fn from_str(input: &str) -> Self;
}

impl FirewallExt for Firewall {
    fn from_str(input: &str) -> Self {
        let mut firewall = Firewall::new();

        // For each line of input...
        for line in input.lines() {
            // Filter whitepsace and punctuation
            let mut iter = line
                .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
                .filter(|s| !s.is_empty());

            // Each line should have depth and range
            let depth = iter.next().unwrap().parse().unwrap();
            let range = iter.next().unwrap().parse().unwrap();
            firewall.insert(depth, Scanner::with_range(range));
        }

        firewall
    }
}

fn part1(input: &str) -> usize {
    let mut firewall = Firewall::from_str(input);

    let mut severity = 0;
    for (&depth, scanner) in firewall.iter_mut() {
        // Update scanner to when our packet enters the layer
        scanner.update(depth);
        if scanner.at_top() {
            severity += depth * scanner.range;
        }
    }

    severity
}

#[test]
fn test_day13() {
    let input =
        "0: 3
         1: 2
         4: 4
         6: 4";
    assert_eq!(part1(&input), 24);
}

pub fn day13(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day13.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
