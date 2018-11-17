use std::fs;
use std::env;
use std::mem;
use std::io::Error;

type Pattern2x2 = ([u8; 4], [u8; 9]);
type Pattern3x3 = ([u8; 9], [u8; 16]);

pub fn day21(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day21.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input, 5));
    //println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &str, steps: usize) -> u8 {
    let (pat2x2, pat3x3) = read_patterns(input);

    //.#.
    //..#
    //###
    let mut front = vec![
        0, 1, 0,
        0, 0, 1,
        1, 1, 1];
    let mut back = Vec::new();

    for i in 0..steps {
        if front.len() % 4 == 0 {
            enhance_2x2(&mut front, &mut back, &pat2x2);
        }
        else {
            enhance_3x3(&mut front, &mut back, &pat3x3);
        }
    }

    front.iter().sum()
}

#[test]
fn test_day21_part1() {
    let input = "\
        ../.# => ##./#../...\n\
        .#./..#/### => #..#/..../..../#..#";
    assert_eq!(part1(&input, 2), 12);
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
            let mut pattern = ([0; 9], [0; 16]);

            // Collect a string, map chars to u8, and collect a Vec
            pattern.0.copy_from_slice(&split.clone().take(3)
                .collect::<Vec<&str>>().concat()
                .chars().map(|c| if c == '#' {1} else {0})
                .collect::<Vec<u8>>());

            // Collect a string, map chars to u8, and collect a Vec
            pattern.1.copy_from_slice(&split.clone().skip(4).take(4)
                .collect::<Vec<&str>>().concat()
                .chars().map(|c| if c == '#' {1} else {0})
                .collect::<Vec<u8>>());

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
    assert_eq!(pat2x2[0],
        ([0, 0,
          0, 1],
         [1, 1, 0,
          1, 0, 0,
          0, 0, 0]));
    assert_eq!(pat3x3[0],
        ([0, 1, 0,
          0, 0, 1,
          1, 1, 1],
         [1, 0, 0, 1,
          0, 0, 0, 0,
          0, 0, 0, 0,
          1, 0, 0, 1]));
}

fn enhance_2x2<'a>(front: &'a mut Vec<u8>, back: &'a mut Vec<u8>, patterns: &[Pattern2x2]) {
    assert_eq!(front.len() % 4, 0);
    let n2 = front.len() / 4;
    back.resize(9 * n2, 0);

    let n = (n2 as f64).sqrt() as usize;

    for by in 0..n {
        for bx in 0..n {
            // Copy 2x2 tile into contiguous memory
            let mut lr = [0; 4];
            for row in 0..2 {
                let (d1, d2) = (row * 2, (row + 1) * 2);
                let (s1, s2) = (((by * 2 + row) * n + bx) * 2, ((by * 2 + row) * n + bx + 1) * 2);
                lr[d1..d2].copy_from_slice(&front[s1..s2]);
            }

            // Find matching 3x3 tile
            let hr = match_pattern_2x2(&lr[..], patterns)
                .expect(&format!("Unable to match pattern {:?}", &lr[..]));
            
            // Copy 3x3 tile into non-contiguous memory
            for row in 0..3 {
                let (d1, d2) = (((by * 3 + row) * n + bx) * 3, ((by * 3 + row) * n + bx + 1) * 3);
                let (s1, s2) = (row * 3, (row + 1) * 3);
                back[d1..d2].copy_from_slice(&hr[s1..s2]);
            }
        }
    }

    // Swap front and back buffers
    mem::swap(front, back);
}

#[test]
fn test_day21_enhance_2x2() {
    let (patterns, _) = read_patterns("../.# => ##./#../...");
    let mut front = vec!
        [1, 0, 0, 1,
         0, 0, 0, 0,
         0, 0, 0, 0,
         1, 0, 0, 1];
    let output =
        [1, 1, 0, 1, 1, 0,
         1, 0, 0, 1, 0, 0,
         0, 0, 0, 0, 0, 0,
         1, 1, 0, 1, 1, 0,
         1, 0, 0, 1, 0, 0,
         0, 0, 0, 0, 0, 0];
    let mut back = Vec::new();
    enhance_2x2(&mut front, &mut back, &patterns);
    assert_eq!(&front[..], &output[..]);
}

fn enhance_3x3<'a>(front: &'a mut Vec<u8>, back: &'a mut Vec<u8>, patterns: &[Pattern3x3]) {
    assert_eq!(front.len() % 9, 0);
    let n2 = front.len() / 9;
    back.resize(16 * n2, 0);

    let n = (n2 as f64).sqrt() as usize;

    for by in 0..n {
        for bx in 0..n {
            // Copy 3x3 tile into contiguous memory
            let mut lr = [0; 9];
            for row in 0..3 {
                let (d1, d2) = (row * 3, (row + 1) * 3);
                let (s1, s2) = (((by * 3 + row) * n + bx) * 3, ((by * 3 + row) * n + bx + 1) * 3);
                lr[d1..d2].copy_from_slice(&front[s1..s2]);
            }

            // Find matching 4x4 tile
            let hr = match_pattern_3x3(&lr[..], patterns)
                .expect(&format!("Unable to match pattern {:?}", &lr[..]));
            
            // Copy 4x4 tile into non-contiguous memory
            for row in 0..4 {
                let (d1, d2) = (((by * 4 + row) * n + bx) * 4, ((by * 4 + row) * n + bx + 1) * 4);
                let (s1, s2) = (row * 4, (row + 1) * 4);
                back[d1..d2].copy_from_slice(&hr[s1..s2]);
            }
        }
    }

    // Swap front and back buffers
    mem::swap(front, back);
}

#[test]
fn test_day21_enhance_3x3() {
    let (_, patterns) = read_patterns(".#./..#/### => #..#/..../..../#..#");
    let mut front = vec!
        [0, 1, 0, 1, 0, 0,
         0, 0, 1, 1, 0, 1,
         1, 1, 1, 1, 1, 0,
         1, 1, 1, 0, 1, 1,
         1, 0, 0, 1, 0, 1,
         0, 1, 0, 0, 0, 1];
    let output =
        [1, 0, 0, 1, 1, 0, 0, 1,
         0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0,
         1, 0, 0, 1, 1, 0, 0, 1,
         1, 0, 0, 1, 1, 0, 0, 1,
         0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0,
         1, 0, 0, 1, 1, 0, 0, 1];
    let mut back = Vec::new();
    enhance_3x3(&mut front, &mut back, &patterns);
    assert_eq!(&front[..], &output[..]);
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

fn match_pattern_3x3<'a>(input: &[u8], patterns: &'a [Pattern3x3]) -> Option<(&'a [u8])> {
    // Pre-compute all 8 transforms of the input
    let transforms = transform_3x3(input);

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
fn test_day21_match_pattern_3x3() {
    let (_, patterns) = read_patterns(".#./..#/### => #..#/..../..../#..#");

    let input =
        [1, 0, 0,
         1, 0, 1,
         1, 1, 0];
    let output =
        [1, 0, 0, 1,
         0, 0, 0, 0,
         0, 0, 0, 0,
         1, 0, 0, 1];
    assert_eq!(match_pattern_3x3(&input, &patterns).unwrap(), &output);

    let input =
        &[1, 1, 0,
          0, 1, 0,
          1, 1, 0];
    assert_eq!(match_pattern_3x3(input, &patterns), None);
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

fn transform_3x3(input: &[u8]) -> Vec<[u8; 9]> {
    let mut transforms = Vec::with_capacity(8);
    transforms.resize(8, [0; 9]);
    transforms[0].copy_from_slice(input);

    for i in 1..4 {
        // Rotate source 90 degrees into target
        transforms[i][2] = transforms[i - 1][0];
        transforms[i][5] = transforms[i - 1][1];
        transforms[i][8] = transforms[i - 1][2];
        transforms[i][1] = transforms[i - 1][3];
        transforms[i][4] = transforms[i - 1][4];
        transforms[i][7] = transforms[i - 1][5];
        transforms[i][0] = transforms[i - 1][6];
        transforms[i][3] = transforms[i - 1][7];
        transforms[i][6] = transforms[i - 1][8];
    }

    for i in 0..4 {
        // Flip source into target
        transforms[i + 4][2] = transforms[i][0];
        transforms[i + 4][1] = transforms[i][1];
        transforms[i + 4][0] = transforms[i][2];
        transforms[i + 4][5] = transforms[i][3];
        transforms[i + 4][4] = transforms[i][4];
        transforms[i + 4][3] = transforms[i][5];
        transforms[i + 4][8] = transforms[i][6];
        transforms[i + 4][7] = transforms[i][7];
        transforms[i + 4][6] = transforms[i][8];
    }

    transforms
}

#[test]
fn test_day21_transform_3x3() {
    assert_eq!(
        transform_3x3(&
            [1, 1, 0,
             0, 1, 0,
             0, 0, 0]),
        vec![
            [1, 1, 0,
             0, 1, 0,
             0, 0, 0],
            [0, 0, 1,
             0, 1, 1,
             0, 0, 0],
            [0, 0, 0,
             0, 1, 0,
             0, 1, 1],
            [0, 0, 0,
             1, 1, 0,
             1, 0, 0],
            [0, 1, 1,
             0, 1, 0,
             0, 0, 0],
            [1, 0, 0,
             1, 1, 0,
             0, 0, 0],
            [0, 0, 0,
             0, 1, 0,
             1, 1, 0],
            [0, 0, 0,
             0, 1, 1,
             0, 0, 1]]);
}
