use std::fs;
use std::env;
use std::io::Error;
use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul};
use std::collections::{BTreeSet, BinaryHeap};

extern crate regex;
use self::regex::{Regex, Captures};

// ==== Vector ====

#[derive(Copy, Clone, PartialEq)]
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

    fn manhattan_len(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn manhattan_cmp(&self, other: &Self) -> Ordering {
        self.manhattan_len().cmp(&other.manhattan_len())
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        Self { x, y, z }
    }
}

// ==== Particle ====

#[derive(Copy, Clone)]
struct Particle {
    acc: Vector,
    vel: Vector,
    pos: Vector,
}

impl Particle {
    fn regex() -> Regex {
        Regex::new(r"([pva])=<\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)>").unwrap()
    }

    fn from_string(string: &str, re: &Regex) -> Self {
        let mut acc = None;
        let mut vel = None;
        let mut pos = None;

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

    fn manhattan_cmp(&self, other: &Self) -> Ordering {
        self.acc.manhattan_cmp(&other.acc)
            .then(self.vel.manhattan_cmp(&other.vel))
            .then(self.pos.manhattan_cmp(&other.pos))
    }

    fn get_position(&self, t: i32) -> Vector {
        let t2 = t * (t + 1) / 2;
        self.pos + self.vel * t + self.acc * t2
    }

    fn does_collide(&self, other: &Self, t: i32) -> bool {
        if t < 0 { return false }
        self.get_position(t) == other.get_position(t)
    }

    fn get_collision(&self, other: &Self) -> Option<(i32)> {
        let rel_acc = self.acc - other.acc;
        let rel_vel = self.vel - other.vel;
        let rel_pos = self.pos - other.pos;

        // Try colliding along each axis until we get non-Always
        let mut collision = get_intersection(rel_acc.x, rel_vel.x, rel_pos.x);
        if let Always = collision { collision = get_intersection(rel_acc.y, rel_vel.y, rel_pos.y) }
        if let Always = collision { collision = get_intersection(rel_acc.z, rel_vel.z, rel_pos.z) }

        match collision {
            Always => Some(0),
            Once(t) => if self.does_collide(other, t) { Some(t) } else { None },
            Twice(t1, t2) => {
                if self.does_collide(other, t1) { Some(t1) }
                else if self.does_collide(other, t2) { Some(t2) }
                else { None }
            },
            Never => None,
        }
    }
}

// ==== Intersection ====

enum Intersection {
    Always,
    Once(i32),
    Twice(i32, i32),
    Never,
}

use self::Intersection::{Always, Once, Twice, Never};

fn get_intersection(rel_acc: i32, rel_vel: i32, rel_pos: i32) -> Intersection {
    if rel_acc == 0 {
        // Handle simplified cases
        if rel_vel == 0 {
            // If both acc and vel are zero, we collide only if the points share the same position
            if rel_pos == 0 { Always } else { Never }
        }
        else {
            // Solve linear equation
            Once(-rel_pos / rel_vel)
        }
    }
    else {
        // Solve quadratic equation: t * (t * acc / 2 + (vel + acc / 2)) + pos == 0
        // Avoid dividing acc by 2 until the end
        let two_a = rel_acc;
        let two_b = 2 * rel_vel + rel_acc;
        let two_c = 2 * rel_pos;
        let two_d_2 = two_b * two_b - 4 * two_a * two_c;

        if two_d_2 < 0 {
            Never
        }
        else if two_d_2 == 0 {
            Once(-two_b / (2 * two_a))
        }
        else {
            let two_d = (two_d_2 as f64).sqrt().round() as i32;
            let t1 = (-two_b - two_d) / (2 * two_a);
            let t2 = (-two_b + two_d) / (2 * two_a);
            Twice(t1, t2)
        }
    }
}

// ==== Collision ====

#[derive(Copy, Clone, Eq)]
struct Collision {
    time: i32,
    first: usize,
    second: usize,
}

impl Collision {
    fn with_info(time: i32, first: usize, second: usize) -> Self {
        Collision { time, first, second }
    }
}

// Implement Ord such that earlier time results in greater order
impl Ord for Collision {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order of variables reversed here
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

// ==== Part 1 ====

fn part1(input: &str) -> Option<usize> {
    let mut closest = None;

    let re = Particle::regex();
    let mut index = 0;
    for line in input.trim().lines() {
        let current = Particle::from_string(line, &re);
        let current_index = index;
        index += 1;

        if let Some((_, previous)) = closest {
            if let Ordering::Greater = current.manhattan_cmp(&previous) {
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

// ==== Part 2 ====

fn part2(input: &str) -> usize {
    // Read all particles into a vector
    let re = Particle::regex();
    let mut particles = Vec::new();
    for line in input.trim().lines() {
        particles.push(Particle::from_string(line, &re));
    }

    // Record all potential collisions in a heap sorted by time
    let mut collisions = BinaryHeap::new();
    for i in 0..particles.len() {
        for j in (i + 1)..particles.len() {
            if let Some(t) = particles[i].get_collision(&particles[j]) {
                collisions.push(Collision::with_info(t, i, j));
            }
        }
    }

    // Initially mark all particles as alive
    let mut alive: BTreeSet<usize> = (0..particles.len()).collect();

    // Iterate over the sorted heap
    let mut stack = Vec::new();
    while let Some(front) = collisions.pop() {
        // Discard if either particle is already dead
        if !alive.contains(&front.first) || !alive.contains(&front.second) { continue }

        // Record the collision
        stack.push(front);

        // Iterate over collisions occuring at the same time
        while let Some(&peek) = collisions.peek() {
            if peek.time > front.time { break }
            let other = collisions.pop().unwrap();

            // Discard if either particle is already dead
            if !alive.contains(&other.first) || !alive.contains(&other.second) { continue }

            // Record the collision
            stack.push(other);
        }

        // Kill particles involved in collisions this step
        for collision in stack.drain(..) {
            alive.remove(&collision.first);
            alive.remove(&collision.second);
        }
    }

    // Return the remainder after all collisions are resolved
    alive.len()
}

#[test]
fn test_day20_part2() {
    let input = "\
        p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\n\
        p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\n\
        p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\n\
        p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>";

    assert_eq!(part2(&input), 1);
}

pub fn day20(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day20.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 2: {}", part2(&input));

    Ok(())
}
