use std::fs;
use std::env;
use std::io;
use std::collections::BTreeMap;

type Firewall = BTreeMap<usize, usize>;

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
            firewall.insert(depth, range);
        }

        firewall
    }
}

// Step through the firewall, computing the severity of the trip
fn part1(firewall: &Firewall) -> usize {
    let mut severity = 0;
    for (&depth, &range) in firewall.iter() {
        // Update scanner to when our packet enters the layer (depth mod cycle length)
        // If the scanner is at the top of the layer, we're caught
        let cycle = (range - 1) * 2;
        if depth % cycle == 0 {
            severity += depth * range;
        }
    }

    severity
}

// Compute the delay required to traverse the firewall without getting caught
fn part2(firewall: &Firewall) -> usize {
    // As-is, we will loop infinitely if there is no solution
    // I think we could detect failure if delay >= the LCM of all ranges?
    let mut delay = 0;
    'top: loop {
        for (&depth, &range) in firewall.iter() {
            // Update scanner to when our packet enters the layer (depth mod cycle length)
            // If the scanner is at the top of the layer, we're caught
            let cycle = (range - 1) * 2;
            if (delay + depth) % cycle == 0 {
                // Increment the delay and start over
                delay += 1;
                continue 'top;
            }
        }

        // We got through without getting caught!
        return delay
    }
}

#[test]
fn test_day13() {
    let input =
        "0: 3
         1: 2
         4: 4
         6: 4";
    let firewall = Firewall::from_str(&input);
    assert_eq!(part1(&firewall), 24);
    assert_eq!(part2(&firewall), 10);
}

pub fn day13(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day13.txt".to_string());
        fs::read_to_string(name)?
    };

    let firewall = Firewall::from_str(&input);
    println!("Part 1: {}", part1(&firewall));
    println!("Part 2: {}", part2(&firewall));

    Ok(())
}
