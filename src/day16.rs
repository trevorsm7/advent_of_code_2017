use std::fs;
use std::env;
use std::io;

fn part1(input: &str, count: usize) -> String {
    // Initialize the elements in ascending order
    let mut order : Vec<u8> = (0..count as u8).collect();

    for command in input.trim().split(',') {
        match &command[..1] {
            "s" => {
                // Parse the number of elements to spin
                let n : usize = command[1..].parse().unwrap();

                // Copy the back and front into a new buffer
                let split_at = count - n;
                let mut new_order = Vec::with_capacity(order.len());
                new_order.extend_from_slice(&order[split_at..]);
                new_order.extend_from_slice(&order[..split_at]);
                order = new_order;
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

    // Map the elements from integers to letters
    order.iter().map(|i| ('a' as u8 + i) as char).collect()
}

#[test]
fn test_day16_part1() {
    assert_eq!(part1("s1,x3/4,pe/b", 5), "baedc");
}

pub fn day16(args: &mut env::Args) -> Result<(), io::Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day16.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input, 16));

    Ok(())
}
