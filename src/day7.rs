use std::fs;
use std::env;
use std::io;
use std::collections::BTreeMap;

struct Tower {
    //name: String,
    //weight: u32,
    children: Vec<String>,
}

impl Tower {
    pub fn with_name(_name: &str, _weight: u32) -> Self {
        Self {
            //name: name.to_string(),
            //weight: weight,
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, name: &str) {
        self.children.push(name.to_string());
    }
}

fn part1(input: &str) -> String {
    let mut towers = BTreeMap::new();

    // Parse the input line-by-line
    for line in input.lines() {
        // Split line with whitespace and punctuation
        let mut iter = line
            .split_terminator(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|s| !s.is_empty());

        // Read name and weight, create new Tower
        let name = iter.next().unwrap();
        let weight = iter.next().unwrap().parse().unwrap();
        let mut tower = Tower::with_name(name, weight);

        // If there are any children, save their names
        for child in iter {
            tower.push_child(child);
        }

        // Store the tower by name in our map
        towers.insert(name.to_string(), tower);
    }

    let mut all_children = Vec::new();

    // Record all names referenced as children
    for tower in towers.values() {
        for name in &tower.children {
            all_children.push(name.to_string());
        }
    }

    // Remove all children from the list
    for name in &all_children {
        towers.remove(name).unwrap();
    }

    // Return the last key in the list
    assert_eq!(towers.len(), 1);
    towers.keys().next().unwrap().to_string()
}

#[test]
fn test_day7_part1() {
    let input =
"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    assert_eq!(part1(&input), "tknk");
}

pub fn day7(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day7.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}
