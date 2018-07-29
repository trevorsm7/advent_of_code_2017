use day10;
use std::env;
use std::io;
use std::collections::BTreeSet;

fn dewit(input: &str) -> (usize, usize) {
    let mut used_count = 0;
    let mut used_set = BTreeSet::new();

    // Number of rows must be a power of 2 up to 128
    const N_ROWS : u32 = 128;
    assert_eq!(N_ROWS & (N_ROWS - 1), 0);

    for row in 0..N_ROWS {
        // Compute the knot hash for this row
        let row_input = format!("{}-{}", input, row);
        let hash = day10::knot_hash(&row_input);

        // We can easily parse this back into a 128-bit integer
        let mut hash_bits = u128::from_str_radix(&hash, 16).expect("Unable to parse hash");

        // Loop over each set bit from the rhs
        while hash_bits > 0 {
            used_count += 1;

            // Get the index of the right-most bit by counting zeros
            let col = hash_bits.trailing_zeros();

            // Use bit-twiddling trick to clear the right-most bit
            // XX1000 - 1 = XX0111
            // XX1000 & XX0111 = XX0000
            hash_bits = hash_bits & (hash_bits - 1);

            // Insert the index (from rhs!) of this bit into our set
            used_set.insert(row * N_ROWS + col);
        }
    }

    let mut region_count = 0;
    let mut to_visit = Vec::new();

    // While there are set bits
    while !used_set.is_empty() {
        region_count += 1;

        // Visit one of the set bits
        to_visit.push(*used_set.iter().next().unwrap());

        // While there are bits to visit
        while !to_visit.is_empty() {
            // Pop an index from the set (depth-first)
            let index = to_visit.pop().unwrap();

            // If the bit at the index is set...
            if used_set.remove(&index) {
                let row = index >> N_ROWS.trailing_zeros();
                let col = index & (N_ROWS - 1);

                // Visit the bit above
                if row > 0 {
                    to_visit.push(index - N_ROWS);
                }

                // Visit the bit below
                if row < N_ROWS - 1 {
                    to_visit.push(index + N_ROWS);
                }

                // Visit the bit to the left
                if col > 0 {
                    to_visit.push(index - 1);
                }

                // Visit the bit to the right
                if col < N_ROWS - 1 {
                    to_visit.push(index + 1);
                }
            }
        }
    }

    (used_count, region_count)
}

#[test]
fn test_day14() {
    assert_eq!(dewit("flqrgnkx"), (8108, 1242));
}

pub fn day14(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from command line
    let input = args.next().unwrap_or("hxtvlmkl".to_string());

    let (part1, part2) = dewit(&input);
    println!("Part 1: {}\nPart 2: {}", part1, part2);

    Ok(())
}
