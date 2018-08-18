use std::fs;
use std::env;
use std::io::Error;

extern crate regex;
use self::regex::{Regex, Captures};

fn manhattan_distance<'t>(caps: &Captures<'t>) -> i32 {
    let x: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let y: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
    let z: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
    x.abs() + y.abs() + z.abs()
}

fn part1(input: &str) -> Option<i32> {
    let mut closest = None;

    let mut current = 0;
    let re = Regex::new(r"([pva])=<\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)>").unwrap();
    for line in input.trim().lines() {
        let mut acc = None;
        let mut vel = None;
        let mut pos = None;

        // Parse each vector on this line
        for caps in re.captures_iter(line) {
            match caps.get(1).unwrap().as_str() {
                "p" => pos = Some(manhattan_distance(&caps)),
                "v" => vel = Some(manhattan_distance(&caps)),
                "a" => acc = Some(manhattan_distance(&caps)),
                _ => {
                    println!("Unhandled: '{}'", caps.get(0).unwrap().as_str());
                    return None;
                },
            }
        }

        // Fail if any of the vectors were missing
        if acc.is_none() || vel.is_none() || pos.is_none() {
            println!("Line '{}' is incomplete", line);
            return None;
        }

        let acc = acc.unwrap();
        let vel = vel.unwrap();
        let pos = pos.unwrap();

        // Find the minimum in order of acc, vel, pos
        let mut is_closest = true;
        if let Some((_, min_acc, min_vel, min_pos)) = closest {
            if acc > min_acc {
                is_closest = false;
            }
            else if acc == min_acc {
                if vel > min_vel {
                    is_closest = false;
                }
                else if vel == min_vel {
                    if pos > min_pos {
                        is_closest = false;
                    }
                }
            }
        }

        if is_closest {
            closest = Some((current, acc, vel, pos));
        }

        current += 1;
    }

    // Return the index of vector closest to origin as t -> infinity
    if let Some((index, _, _, _)) = closest { Some(index) } else { None }
}

#[test]
fn test_day20_part1() {
    let input = "\
        p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\n\
        p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";

    assert_eq!(part1(&input), Some(0));
}

pub fn day20(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day20.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input).unwrap());

    Ok(())
}
