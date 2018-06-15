mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::error::Error;
use std::process::exit;

fn main() {
    let mut args = std::env::args();

    let name = args
        .next()
        .expect("expect application name");

    let day = args
        .next()
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0);

    let result = match day {
        1 => day1::day1(&mut args),
        2 => day2::day2(&mut args),
        3 => day3::day3(&mut args),
        4 => day4::day4(&mut args),
        5 => day5::day5(&mut args),
        _ => {
            println!("Usage: {} day", name);
            exit(1);
        }
    };

    if let Err(e) = result {
        println!("Error: {}", e);

        while let Some(e) = e.cause() {
            println!("Caused by: {}", e);
        }
    };
}
