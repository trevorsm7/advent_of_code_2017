use std::fs;
use std::env;
use std::io;

fn part1(size: usize, input: &str) -> usize {
    assert!(size >= 2);

    // Initialize list with values incrementing from zero
    let mut list: Vec<usize> = (0..size).collect();
    let mut position = 0;
    let mut skip = 0;

    // For each length in the input...
    let lengths = input
        .trim_right() // discard EOL whitespace
        .split(',')
        .map(|tok| tok.parse::<usize>().expect("expected a number"));
    for length in lengths {
        assert!(length <= size);

        // Reverse [length] elements
        // NOTE this works for length=1, but we can skip it
        if length > 1 {
            let mut i = position;
            let mut j = (position + length - 1) % size;
            while i != j {
                // Swap each end of the sublist
                let swap = list[i];
                list[i] = list[j];
                list[j] = swap;

                // Pop from each end of the sublist
                i = (i + 1) % size;
                if j == i {
                    // Stop when we swap the last two elements
                    break;
                }
                j = (size + j - 1) % size;
            }
        }

        // Update position and skip size
        position = (position + length + skip) % size;
        skip += 1;
    }

    list[0] * list[1]
}

#[test]
fn test_day10() {
    assert_eq!(part1(5, "3,4,1,5"), 12);
}

pub fn day10(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day10.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(256, &input));

    Ok(())
}
