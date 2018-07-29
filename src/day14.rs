use day10;
use std::env;
use std::io;

fn part1(input: &str) -> usize {
    let mut used = 0;

    for row in 0..128 {
        // Compute the knot hash for this row
        // NOTE knot_hash could just return a 128 bit integer, but I'm sticking to the prompt
        let row_input = format!("{}-{}", input, row);
        let hash = day10::knot_hash(&row_input);

        for c in hash.chars() {
            // Use a lookup table rather than count the bits each time
            used += match c {
                '1' | '2' | '4' | '8' => 1,
                '3' | '5' | '6' | '9' | 'a' | 'c' => 2,
                '7' | 'b' | 'd' | 'e' => 3,
                'f' => 4,
                _ => 0,
            };
        }
    }

    used
}

#[test]
fn test_day14_part1() {
    assert_eq!(part1("flqrgnkx"), 8108);
}

pub fn day14(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from command line
    let input = args.next().unwrap_or("hxtvlmkl".to_string());

    println!("Knot hash: {}", part1(&input));

    Ok(())
}
