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

        // Swap front and back buffers
        mem::swap(&mut front, &mut back);
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

            // Map to 2x2 block linear memory
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

fn enhance_2x2(front: &Vec<u8>, back: &mut Vec<u8>, patterns: &[Pattern2x2]) {
    let n = front.len() / 4;
    back.resize(9 * n, 0);

    for i in 0..n {
        let (f1, f2) = (i * 4, (i + 1) * 4);
        let (b1, b2) = (i * 9, (i + 1) * 9);
        back[b1..b2].copy_from_slice(
            match_pattern_2x2(&front[f1..f2], patterns)
                .expect(&format!("Unable to match pattern {:?}", &front[f1..f2])));
    }
}

#[test]
fn test_day21_enhance_2x2() {
    let (patterns, _) = read_patterns("../.# => ##./#../...");
    let front = vec![
        1, 0,
        0, 0,
        0, 1,
        0, 0,
        0, 0,
        1, 0,
        0, 0,
        0, 1];
    let mut back = Vec::new();
    enhance_2x2(&front, &mut back, &patterns);
    for i in 0..4 {
        let (a, b) = (i * 9, (i + 1) * 9);
        assert_eq!(
            &back[a..b],
            &[1, 1, 0,
              1, 0, 0,
              0, 0, 0]);
    }
}

fn match_pattern_2x2<'a>(input: &[u8], patterns: &'a [Pattern2x2]) -> Option<&'a [u8]> {
    // Pre-compute all 8 transforms of the input
    let transforms = transform_2x2(input);

    // Try to match each transform with each pattern
    for (ref lr, ref hr) in patterns {
        for &source in &transforms {
            if *lr == source {
                return Some(hr);
            }
        }
    }

    None
}

#[test]
fn test_day21_match_pattern_2x2() {
    let (patterns, _) = read_patterns("../.# => ##./#../...");

    let input =
        &[0, 1,
          0, 0];
    let output =
        &[1, 1, 0,
          1, 0, 0,
          0, 0, 0];
    assert_eq!(match_pattern_2x2(input, &patterns).unwrap(), output);

    let input =
        &[1, 1,
          0, 0];
    assert_eq!(match_pattern_2x2(input, &patterns), None);
}

fn transform_2x2(input: &[u8]) -> Vec<[u8; 4]> {
    let mut transforms = Vec::with_capacity(8);
    transforms.resize(8, [0; 4]);
    transforms[0].copy_from_slice(input);

    for i in 1..4 {
        // Rotate source 90 degrees into target
        transforms[i][1] = transforms[i - 1][0];
        transforms[i][3] = transforms[i - 1][1];
        transforms[i][0] = transforms[i - 1][2];
        transforms[i][2] = transforms[i - 1][3];
    }

    for i in 0..4 {
        // Flip source into target
        transforms[i + 4][1] = transforms[i][0];
        transforms[i + 4][0] = transforms[i][1];
        transforms[i + 4][3] = transforms[i][2];
        transforms[i + 4][2] = transforms[i][3];
    }

    transforms
}

#[test]
fn test_day21_transform_2x2() {
    assert_eq!(
        transform_2x2(&
            [1, 1,
             0, 0]),
        vec![
            [1, 1,
             0, 0],
            [0, 1,
             0, 1],
            [0, 0,
             1, 1],
            [1, 0,
             1, 0],
            [1, 1,
             0, 0],
            [1, 0,
             1, 0],
            [0, 0,
             1, 1],
            [0, 1,
             0, 1]]);
    assert_eq!(
        transform_2x2(&
            [1, 0,
             1, 0]),
        vec![
            [1, 0,
             1, 0],
            [1, 1,
             0, 0],
            [0, 1,
             0, 1],
            [0, 0,
             1, 1],
            [0, 1,
             0, 1],
            [1, 1,
             0, 0],
            [1, 0,
             1, 0],
            [0, 0,
             1, 1]]);
}
