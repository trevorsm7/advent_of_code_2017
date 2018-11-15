use std::fs;
use std::env;
use std::mem;
use std::io::Error;

type Pattern2x2 = ([u8; 4], [u8; 9]);
type Pattern3x3 = ([u8; 9], [u8; 8], [u8; 8]);

pub fn day21(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day21.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> u32 {
    let (pat2x2, pat3x3) = read_patterns(input);

    //.#.
    //..#
    //###
    let mut front = vec![
        0, 1, 0,
        0, 0, 1,
        1, 1, 1];
    let mut back = Vec::new();

    for i in 0..5 {
        if front.len() % 4 == 0 {
            enhance_2x2(&mut front, &mut back, &pat2x2);
        }
        else {
            // TODO 3x3
        }
    }

    0
}

fn read_patterns(input: &str) -> (Vec<Pattern2x2>, Vec<Pattern3x3>) {
    let mut pat2x2 = Vec::new();
    let mut pat3x3 = Vec::new();

    for line in input.trim().lines() {
        let split = line.split(|c| c == '/' || c == ' ');
        let count = split.clone().count();
        if count == 6 {
            let mut pattern = ([0; 4], [0; 9]);

            // Collect a string, map chars to u8, and collect a Vec
            pattern.0.copy_from_slice(&split.clone().take(2)
                .collect::<Vec<&str>>().concat()
                .chars().map(|c| if c == '#' {1} else {0})
                .collect::<Vec<u8>>());

            // Collect a string, map chars to u8, and collect a Vec
            pattern.1.copy_from_slice(&split.clone().skip(3).take(3)
                .collect::<Vec<&str>>().concat()
                .chars().map(|c| if c == '#' {1} else {0})
                .collect::<Vec<u8>>());

            pat2x2.push(pattern);
        }
        else if count == 8 {
            let mut pattern = ([0; 9], [0; 8], [0; 8]);

            // Collect a string, map chars to u8, and collect a Vec
            pattern.0.copy_from_slice(&split.clone().take(3)
                .collect::<Vec<&str>>().concat()
                .chars().map(|c| if c == '#' {1} else {0})
                .collect::<Vec<u8>>());

            // Collect a string, map chars to u8, and collect a Vec
            let pitched = split.clone().skip(4).take(4)
                .collect::<Vec<&str>>().concat()
                .chars().map(|c| if c == '#' {1} else {0})
                .collect::<Vec<u8>>();

            // Map to block linear memory
            pattern.1[0..2].copy_from_slice(&pitched[0..2]);
            pattern.1[4..6].copy_from_slice(&pitched[2..4]);
            pattern.1[2..4].copy_from_slice(&pitched[4..6]);
            pattern.1[6..8].copy_from_slice(&pitched[6..8]);
            pattern.2[0..2].copy_from_slice(&pitched[8..10]);
            pattern.2[4..6].copy_from_slice(&pitched[10..12]);
            pattern.2[2..4].copy_from_slice(&pitched[12..14]);
            pattern.2[6..8].copy_from_slice(&pitched[14..16]);
            
            pat3x3.push(pattern);
        }
        else {
            println!("warning: unhandled count {} in \"{}\"", count, line);
        }
    }

    (pat2x2, pat3x3)
}

#[test]
fn test_day21_read_patterns() {
    let input = "\
        ../.# => ##./#../...\n\
        .#./..#/### => #..#/..../..../#..#";
    let (pat2x2, pat3x3) = read_patterns(&input);
    assert_eq!(pat2x2[0], (
        [0, 0,
         0, 1],
        [1, 1, 0,
         1, 0, 0,
         0, 0, 0]));
    assert_eq!(pat3x3[0], (
        [0, 1, 0,
         0, 0, 1,
         1, 1, 1],
        [1, 0,
         0, 0,
         0, 1,
         0, 0],
        [0, 0,
         1, 0,
         0, 0,
         0, 1]));
}

// NOTE we probably want to use 2x2/3x3 block-linear layout
fn enhance_2x2(front: &mut Vec<u8>, back: &mut Vec<u8>, patterns: &[Pattern2x2]) {
    let n = front.len() / 4;
    back.resize(9 * n, 0);

    for i in 0..n {
        let (f1, f2) = (n * 4, (n + 1) * 4);
        let (b1, b2) = (n * 9, (n + 1) * 9);
        match_pattern_2x2(&front[f1..f2], &mut back[b1..b2], patterns);
    }

    // TODO why can't I borrow mutably?
    //mem::swap(front, &mut back);
}

fn match_pattern_2x2(input: &[u8], output: &mut [u8], patterns: &[Pattern2x2]) {
    // Pre-compute all 8 transforms of the input
    let transforms = transform_2x2(input);

    // Try to match each transform with each pattern
    for &(lr, hr) in patterns {
        for &source in &transforms {
            if lr == source {
                // Copy the matching high res stamp
                output.copy_from_slice(&hr);
                return;
            }
        }
    }

    // TODO return an error if no match found?
    println!("Warning: no match found for {:?}", input);
}

/*#[test]
fn test_day21_match_pattern_2x2() {

}*/

fn transform_2x2(input: &[u8]) -> Vec<[u8; 4]> {
    let mut transforms = Vec::with_capacity(8);
    transforms.resize(8, [0; 4]);

    // TODO generate transforms

    transforms
}

/*#[test]
fn test_day21_transform_2x2() {

}*/
