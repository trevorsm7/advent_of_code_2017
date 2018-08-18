use std::fs;
use std::env;
use std::io::Error;

fn is_valid(grid: &[char], rows: i32, cols: i32, x: i32, y: i32) -> bool {
    if x < 0 || x >= cols || y < 0 || y >= rows {
        return false;
    }

    // Assume valid if non-whitespace
    !grid[(y * cols + x) as usize].is_whitespace()
}

fn part1(input: &str) -> String {
    // Read rows as a 2D grid
    let mut rows = 0;
    let mut cols = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        if cols == 0 {
            cols = line.chars().count() as i32;
        }
        else if cols != line.chars().count() as i32 {
            println!("Warning! Skipping line '{}'", line);
            continue;
        }
        grid.extend(line.chars());
        rows += 1;
    }

    // Find start along top row
    let mut x = 0;
    while x < cols {
        if grid[x as usize] == '|' { break; }
        x += 1;
    }

    if x == cols {
        println!("Error: unable to find start");
        return "".to_string();
    }

    let mut y = 0;
    let mut dx = 0;
    let mut dy = 1;

    // Follow lines while pushing letters
    let mut stack = Vec::new();

    loop {
        // If we can't move forward, try to turn
        if !is_valid(&grid, rows, cols, x + dx, y + dy) {
            if is_valid(&grid, rows, cols, x + dy, y - dx) {
                // Turn left
                let (dxn, dyn) = (dy, -dx);
                dx = dxn;
                dy = dyn;
            }
            else if is_valid(&grid, rows, cols, x - dy, y + dx) {
                // Turn right
                let (dxn, dyn) = (-dy, dx);
                dx = dxn;
                dy = dyn;
            }
            else {
                // Reached a dead end; return the stack
                return stack.iter().collect();
            }
        }

        // Move forward
        x += dx;
        y += dy;

        // If we visit a letter, push it on the stack
        let c = grid[(y * cols + x) as usize];
        if c.is_alphabetic() {
            stack.push(c);
        }
    }
}

#[test]
fn test_day19_part1() {
    let input = concat!(
        "     |          \n",
        "     |  +--+    \n",
        "     A  |  C    \n",
        " F---|----E|--+ \n",
        "     |  |  |  D \n",
        "     +B-+  +--+ \n",
        "                ");

    assert_eq!(&part1(&input), "ABCDEF");
}

pub fn day19(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day19.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));

    Ok(())
}
