use std::{fs, env, cmp};
use std::io::Error;
use std::collections::HashSet;

type Point = (i32, i32);
type Grid = HashSet<Point>;

// Up, right, down, left; +1 clockwise, -1 counter-clockwise
const HEADINGS: [Point; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn day22(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day22.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input, 10000));
    println!("Part 2: {}", part2(&input, 10000000));

    Ok(())
}

fn part1(input: &str, bursts: usize) -> usize {
    let mut grid = Grid::from_string(input);

    let mut carrier = (0, 0);
    let mut heading = 0;

    let mut infections = 0;
    for _ in 0..bursts {
        // If node is infected
        if grid.remove(&carrier) {
            // Turn right, clean node
            heading = (heading + 1) % 4;
        }
        else {
            // Turn left, infect node
            heading = (heading + 3) % 4;
            grid.insert(carrier);
            infections += 1;
        }

        // Advance carrier one step
        carrier.0 += HEADINGS[heading].0;
        carrier.1 += HEADINGS[heading].1;
    }

    infections
}

#[test]
fn test_day22_part1() {
    let input = "\
        ..#\n\
        #..\n\
        ...";
    assert_eq!(part1(&input, 7), 5);
    assert_eq!(part1(&input, 70), 41);
    assert_eq!(part1(&input, 10000), 5587);
}

fn part2(input: &str, bursts: usize) -> usize {
    let mut weakened = Grid::new();
    let mut infected = Grid::from_string(input);
    let mut flagged = Grid::new();

    let mut carrier = (0, 0);
    let mut heading = 0;

    let mut infections = 0;
    for _ in 0..bursts {
        if weakened.remove(&carrier) {
            // Do not turn, infect node
            infected.insert(carrier);
            infections += 1;
        }
        else if infected.remove(&carrier) {
            // Turn right, flag node
            heading = (heading + 1) % 4;
            flagged.insert(carrier);
        }
        else if flagged.remove(&carrier) {
            // Reverse direction, clean node
            heading = (heading + 2) % 4;
        }
        else {
            // Turn left, weaken node
            heading = (heading + 3) % 4;
            weakened.insert(carrier);
        }

        // Advance carrier one step
        carrier.0 += HEADINGS[heading].0;
        carrier.1 += HEADINGS[heading].1;
    }

    infections
}

#[test]
fn test_day22_part2() {
    let input = "\
        ..#\n\
        #..\n\
        ...";
    assert_eq!(part2(&input, 100), 26);
    assert_eq!(part2(&input, 10000000), 2511944);
}

trait GridExt {
    fn from_string(input: &str) -> Self;
    fn to_string(&self) -> String;
}

impl GridExt for Grid {
    fn from_string(input: &str) -> Self {
        let mut grid = HashSet::new();

        // For each row
        let lines = input.trim().lines();
        let h = lines.clone().count() as i32;
        let mut y = (1 - h) / 2;
        for line in lines {
            // For each col
            let chars = line.chars();
            let w = chars.clone().count() as i32;
            let mut x = (1 - w) / 2;
            for c in chars {
                // Insert (x, y) for each infected node
                if c == '#' {
                    grid.insert((x, y));
                }
                x += 1;
            }
            y += 1;
        }

        grid
    }

    fn to_string(&self) -> String {
        // Find the most extreme points
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for &(x, y) in self {
            min_x = cmp::min(min_x, x);
            max_x = cmp::max(max_x, x);
            min_y = cmp::min(min_y, y);
            max_y = cmp::max(max_y, y);
        }

        // Create an empty symmetric grid centered at 0, 0
        let w = 2 * cmp::max(max_x, -min_x) + 1;
        let h = 2 * cmp::max(max_y, -min_y) + 1;
        let mut rows = vec![vec!['.'; w as usize]; h as usize];

        // Plot each point in the grid
        for &(x, y) in self {
            let row = (y + (h - 1) / 2) as usize;
            let col = (x + (w - 1) / 2) as usize;
            rows[row][col] = '#';
        }

        // Collect grid into Vec<String>, then join into String
        rows.iter().map(|v| v.iter().collect())
            .collect::<Vec<String>>().join("\n")
    }
}

#[test]
fn test_day22_grid() {
    let input = "\
        ##..#\n\
        #..##\n\
        ..#.#\n\
        .##..\n\
        ###.#";
    let grid = Grid::from_string(&input);
    assert_eq!(&grid.to_string(), input);
}
