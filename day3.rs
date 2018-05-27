use std::env;

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

fn main() -> Result<(), std::io::Error> {
    // Expect input to be passed as first arg
    let input = env::args()
        .skip(1)
        .next()
        .expect("expected argument")
        .parse()
        .expect("expected argument to be a number");

    println!("Part 1: {}", part1(input));

    Ok(())
}
