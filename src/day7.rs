use std::fs;
use std::env;
use std::io;
use std::collections::HashMap;

#[derive(Clone)]
struct Tower {
    //name: String,
    weight: u32,
    children: Vec<String>,
}

impl Tower {
    pub fn with_name(_name: &str, weight: u32) -> Self {
        Self {
            //name: name.to_string(),
            weight: weight,
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, name: &str) {
        self.children.push(name.to_string());
    }
}

type TowerMap = HashMap<String, Tower>;

trait HashMapExt {
    fn from_input(input: &str) -> TowerMap;
    fn find_root(&self) -> String;
    fn find_imbalance(&self, &str) -> Result<(u32, u32), u32>;
}

impl HashMapExt for TowerMap {
    fn from_input(input: &str) -> TowerMap {
        let mut tower_map = Self::new();

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
            tower_map.insert(name.to_string(), tower);
        }

        tower_map
    }

    fn find_root(&self) -> String {
        let mut towers = self.clone();
        let mut all_children = Vec::with_capacity(self.len());

        // Record all names referenced as children
        for tower in towers.values() {
            for name in &tower.children {
                all_children.push(name.to_string());
            }
        }

        // Remove all children from the set
        for name in &all_children {
            towers.remove(name).unwrap();
        }

        // Return the last key in the set
        assert_eq!(towers.len(), 1);
        towers.keys().next().unwrap().to_string()
    }

    fn find_imbalance(&self, root_name: &str) -> Result<(u32, u32), u32> {
        // Get the root tower
        let tower = self.get(root_name).unwrap();

        // A state machine to find a consensus between values
        enum Consensus<T> {
            None,
            Propose((T, T)),
            Disagree(((T, T), (T, T))),
            Agree(T)
        }

        // Sum weight over each child
        let mut sum_children = 0;
        let mut consensus = Consensus::None;
        for name in &tower.children {
            // Return immediately if we found an imbalance
            let weights = self.find_imbalance(name)?;
            let total = weights.0 + weights.1;
            sum_children += total;

            // Update the state machine, looking for an imbalance
            consensus = match consensus {
                Consensus::None => Consensus::Propose(weights),
                Consensus::Propose(w) if (w.0 + w.1) == total => Consensus::Agree(total),
                Consensus::Propose(other) => Consensus::Disagree((other, weights)),
                Consensus::Agree(w) if w == total => Consensus::Agree(w),
                Consensus::Agree(w) => return Err(w - weights.1),
                Consensus::Disagree((a, b)) if (a.0 + a.1) == total => return Err(total - b.1),
                Consensus::Disagree((a, b)) if (b.0 + b.1) == total => return Err(total - a.1),
                Consensus::Disagree(_) => panic!("multiple imbalances detected"),
            }
        }

        // Just panic if we're unable to resolve which child is imbalanced
        if let Consensus::Disagree(_) = consensus {
            panic!("unable to resolve imbalance");
        }

        Ok((tower.weight, sum_children))
    }
}

fn dewit(input: &str) -> (String, u32) {
    let towers = TowerMap::from_input(input);
    let root = towers.find_root();
    let weight = towers.find_imbalance(&root).unwrap_err();
    (root, weight)
}

#[test]
fn test_day7() {
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
    assert_eq!(dewit(&input), ("tknk".to_string(), 60));
}

pub fn day7(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day7.txt".to_string());
        fs::read_to_string(name)?
    };

    let (part1, part2) = dewit(&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
