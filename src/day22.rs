use std::{fs, env, cmp};
use std::io::Error;
use std::collections::HashSet;

pub fn day22(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day22.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::from_string(input);
    println!("{}", grid.to_string());

    0
}

type Point = (i32, i32);

struct Grid {
    grid: HashSet<Point>,
}

impl Grid {
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
        
        Self { grid }
    }

    fn to_string(&self) -> String {
        // Find the most extreme points
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for &(x, y) in &self.grid {
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
        for &(x, y) in &self.grid {
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
