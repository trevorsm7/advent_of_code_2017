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

fn part1(input: &str) -> usize {
    input
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
        .fold(HexCoord(0, 0), |pos, step| pos + step)
        // Return the absolute number of steps from the origin
        .distance()
}

#[test]
fn test_day11_part1() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}

pub fn day11(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day11.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
