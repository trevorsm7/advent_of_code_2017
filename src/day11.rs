use std::fs;
use std::env;
use std::io;
use std::ops;

struct HexCoord(isize, isize);

impl HexCoord {
    fn distance(&self) -> usize {
        let HexCoord(x, y) = self;
        let (x, y) = (x.abs(), y.abs());
        // Y must be greater than X to contribute to distance
        let y = (y - x).max(0) / 2;
        (x + y) as usize
    }
}

impl ops::Add for HexCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        HexCoord(self.0 + other.0, self.1 + other.1)
    }
}

fn dewit(input: &str) -> (usize, usize) {
    let mut max = 0;

    let dist = input
        // Trim and split tokens
        .trim_right()
        .split(',')
        // Map each token to a step on the hex grid
        .map(|tok| match tok {
            "n" => HexCoord(0, 2),
            "s" => HexCoord(0, -2),
            "nw" => HexCoord(-1, 1),
            "ne" => HexCoord(1, 1),
            "sw" => HexCoord(-1, -1),
            "se" => HexCoord(1, -1),
            _ => panic!(format!("Invalid token '{}'", tok)),
        })
        // Sum each step to get the new position
        .fold(HexCoord(0, 0), |pos, step| {
            let pos = pos + step;
            max = max.max(pos.distance());
            pos
        })
        // Return the absolute number of steps from the origin
        .distance();

    (dist, max)
}

#[test]
fn test_day11() {
    assert_eq!(dewit("ne,ne,ne"), (3, 3));
    assert_eq!(dewit("ne,ne,sw,sw"), (0, 2));
    assert_eq!(dewit("ne,ne,s,s"), (2, 2));
    assert_eq!(dewit("se,sw,se,sw,sw"), (3, 3));
}

pub fn day11(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day11.txt".to_string());
        fs::read_to_string(name)?
    };

    let (part1, part2) = dewit(&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
