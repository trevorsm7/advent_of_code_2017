use std::fs;
use std::env;
use std::io::Error;
use std::cmp::Ordering;

extern crate regex;
use self::regex::{Regex, Captures};

#[derive(Copy, Clone)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn from_caps<'t>(caps: &Captures<'t>) -> Self {
        let x: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let y: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let z: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
        Self { x, y, z }
    }

    fn manhattan_length(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.manhattan_length().partial_cmp(&other.manhattan_length())
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.manhattan_length() == other.manhattan_length()
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct Particle {
    acc: Vector,
    vel: Vector,
    pos: Vector,
}

impl Particle {
    fn from_string(string: &str) -> Self {
        let mut acc = None;
        let mut vel = None;
        let mut pos = None;

        let re = Regex::new(r"([pva])=<\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)>").unwrap();
        for caps in re.captures_iter(string) {
            match caps.get(1).unwrap().as_str() {
                "p" => pos = Some(Vector::from_caps(&caps)),
                "v" => vel = Some(Vector::from_caps(&caps)),
                "a" => acc = Some(Vector::from_caps(&caps)),
                _ => {
                    panic!(format!("Unhandled: '{}'", caps.get(0).unwrap().as_str()));
                },
            }
        }

        let acc = acc.expect("Failed to parse acceleration");
        let vel = vel.expect("Failed to parse velocity");
        let pos = pos.expect("Failed to parse position");
        Self { acc, vel, pos }
    }
}

fn part1(input: &str) -> Option<i32> {
    let mut closest = None;

    let mut index = 0;
    for line in input.trim().lines() {
        let current = Particle::from_string(line);

        let current_index = index;
        index += 1;

        if let Some((_, previous)) = closest {
            if current >= previous {
                continue;
            }
        }

        closest = Some((current_index, current));
    }

    // Return the index of vector closest to origin as t -> infinity
    if let Some((index, _)) = closest { Some(index) } else { None }
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
