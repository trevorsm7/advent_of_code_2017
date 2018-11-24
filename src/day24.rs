use std::{fs, env};
use std::io::Error;

type Component = (u16, u16);

pub fn day24(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day24.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    0
}

#[test]
fn test_day24_part1() {
    let input = "\
        0/2\n\
        2/2\n\
        2/3\n\
        3/4\n\
        3/5\n\
        0/1\n\
        10/1\n\
        9/10\n";
    assert_eq!(part1(&input), 31);
}

fn read_components(input: &str) -> Vec<Component> {
    input.trim().lines()
        .map(|line| {
            let mut it = line.split('/');
            let a = it.next().unwrap().parse().unwrap();
            let b = it.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect::<Vec<Component>>()
}

#[test]
fn test_day24_read_components() {
    let input = "\
        0/2\n\
        1/3\n\
        2/4\n";
    assert_eq!(&read_components(&input),
        &[(0, 2), (1, 3), (2, 4)]);
}
