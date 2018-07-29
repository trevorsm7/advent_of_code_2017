use std::env;
use std::io;

fn part1(mut input1: u64, mut input2: u64) -> u64 {
    const GEN1 : u64 = 16807;
    const GEN2 : u64 = 48271;
    const REM : u64 = 2147483647;

    let mut matches = 0;

    // Underscores in numeric literals are nice here
    for _ in 0..40_000_000 {
        // Really? This was not remotely interesting to implement
        input1 = (input1 * GEN1) % REM;
        input2 = (input2 * GEN2) % REM;
        if input1 & 0xFFFF == input2 & 0xFFFF {
            matches += 1;
        }
    }

    matches
}

#[test]
fn test_day15_part1() {
    assert_eq!(part1(65, 8921), 588);
}

pub fn day15(args: &mut env::Args) -> Result<(), io::Error> {
    // Read input from command line
    let input1 = args.next().unwrap_or("618".to_string()).parse().unwrap();
    let input2 = args.next().unwrap_or("814".to_string()).parse().unwrap();

    println!("Part 1: {}", part1(input1, input2));

    Ok(())
}
