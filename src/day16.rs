use std::fs;
use std::env;
use std::io;
use std::mem;

fn apply_pattern(order: &mut Vec<u8>, input: &str) {
    let mut swap = Vec::with_capacity(order.len());

    for command in input.trim().split(',') {
        match &command[..1] {
            "s" => {
                // Parse the number of elements to spin
                let n : usize = command[1..].parse().unwrap();
                let split_at = order.len() - n;

                // Copy the back and front into the back buffer
                swap.extend_from_slice(&order[split_at..]);
                swap.extend_from_slice(&order[..split_at]);

                // Swap the buffer pointers and clear the new back buffer
                mem::swap(order, &mut swap);
                swap.clear();
            },
            "x" => {
                // Parse two indices separated by /
                let mut iter = command[1..].split('/')
                    .map(|n| n.parse::<usize>().unwrap());
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();

                // Swap the elements at these indices
                let tmp = order[a];
                order[a] = order[b];
                order[b] = tmp;
            },
            "p" => {
                // Parse two names separated by /
                // NOTE (ab)using base-36 conversion minus 10 to map a,b.. to 0,1..
                let mut iter = command[1..].split('/')
                    .map(|c| u8::from_str_radix(c, 36).unwrap() - 10);
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();

                // Find the position of these elements
                let pa = order.iter().position(|&n| n == a).unwrap();
                let pb = order.iter().position(|&n| n == b).unwrap();

                // Swap the elements
                let tmp = order[pa];
                order[pa] = order[pb];
                order[pb] = tmp;
            },
            _ => panic!("invalid command"),
        }
    }
}

fn generate_order(input: &str, count: u8, iterations: u32) -> String {
    // Initialize the elements in ascending order
    let init : Vec<u8> = (0..count).collect();

    // Apply the pattern in a loop
    let mut order = init.to_vec();
    for i in 0..iterations {
        apply_pattern(&mut order, input);

        // If we find a cycle, we can skip to the remainder of the final cycle
        if &order == &init {
            println!("Found cycle after {} iterations", i + 1);
            let remainder = iterations % (i + 1);
            for _ in 0..remainder {
                apply_pattern(&mut order, input);
            }
            break;
        }
    }

    // Map the elements from integers to letters
    order.iter().map(|i| ('a' as u8 + i) as char).collect()
}

#[test]
fn test_day16_part1() {
    assert_eq!(generate_order("s1,x3/4,pe/b", 5, 1), "baedc");
}

#[test]
fn test_day16_part2() {
    // NOTE the example pattern has a cycle after 4 iterations
    assert_eq!(generate_order("s1,x3/4,pe/b", 5, 2), "ceadb");
    assert_eq!(generate_order("s1,x3/4,pe/b", 5, 4), "abcde");
    assert_eq!(generate_order("s1,x3/4,pe/b", 5, 9), "baedc");
    assert_eq!(generate_order("s1,x3/4,pe/b", 5, 14), "ceadb");
}

pub fn day16(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day16.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", generate_order(&input, 16, 1));
    println!("Part 2: {}", generate_order(&input, 16, 1_000_000_000));

    Ok(())
}
