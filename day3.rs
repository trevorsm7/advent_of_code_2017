use std::env;
use std::mem;

fn part1(input: u32) -> u32 {
    // Handle trivial case
    if input <= 1 {return 0}

    // Compute the radial distance (Chebyshev distance)
    let radius = ((((input - 1) as f64).sqrt() - 1.) * 0.5).floor() as u32 + 1;

    // Compute the tangential distance
    let tangent = {
        // Compute the number of squares within this radius
        let squares = {
            let t = radius * 2 + 1;
            t * t
        };

        // Length of each edge, minus 1
        let edge_len = radius * 2;

        // Get the distance from input to the middle of the edge
        let offset = ((squares - input) % edge_len) as i32;
        let middle = radius as i32;
        (offset - middle).abs() as u32
    };

    // Taxicab distance is radial distance plus tangential distance
    radius + tangent
}

fn part2(input: u32) -> u32 {
    // Initialize the inner ring to a single 1
    let mut inner = vec![1];
    let mut outer = vec![];

    // Iterate outward over concentric rings
    let mut ring = 1;
    loop {
        // Compute num elements along each edge and around the ring
        let edge_n = ring * 2;
        let ring_n = edge_n * 4;

        // NOTE these are invalid for the 0th ring, so we treat that as a special case
        let inner_edge_n = (ring - 1) * 2;
        let inner_ring_n = inner_edge_n * 4;

        // Iterate over each index in the ring
        for ring_i in 0..ring_n {
            let edge_i = ring_i % edge_n;
            let offset = (ring_i / edge_n) * inner_edge_n;

            // ==== Get adjacent values in outer ring ====
            // The previous index is always adjacent (if it exists!)
            let mut val = *outer.last().unwrap_or(&0);

            // The 1st index along each edge is adjacent to the 2nd to last index
            if edge_i == 0 && ring_i > 1 {
                val += outer[ring_i - 2];
            };

            // The last 2 indices in the ring are adjacent to the 1st
            if ring_n - ring_i <= 2 {
                val += outer[0];
            };

            // ==== Get adjacent values in inner ring ====
            if ring == 1 {
                // Special case for 0th ring
                val += inner[0];
            } else {
                if edge_i == edge_n - 1 {
                    // This is a corner, so the corner of the inner ring is adjacent
                    val += inner[offset + inner_edge_n - 1];
                } else {
                    // Get the adjacent square first
                    let adjacent = offset + edge_i + inner_ring_n - 1;
                    val += inner[adjacent % inner_ring_n];

                    // Get the backward diagonal
                    if edge_i > 0 {
                        val += inner[(adjacent - 1) % inner_ring_n];
                    }

                    // Get the forwards diagonal
                    if edge_i < edge_n - 2 {
                        val += inner[(adjacent + 1) % inner_ring_n];
                    }
                };
            };

            // Return the first value larger than input
            if val > input {
                return val;
            };

            // Cache the value in the outer ring
            outer.push(val);
        };

        // Swap outer to inner for next ring (swaps refs, does not move elements)
        mem::swap(&mut inner, &mut outer);
        outer.clear();
        ring += 1;
    };
}

fn main() -> Result<(), std::io::Error> {
    // Expect input to be passed as first arg
    let input = env::args()
        .skip(1)
        .next()
        .expect("expected argument")
        .parse()
        .expect("expected argument to be a number");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    Ok(())
}
