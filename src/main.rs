mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

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
        6 => day6::day6(&mut args),
        7 => day7::day7(&mut args),
        8 => day8::day8(&mut args),
        9 => day9::day9(&mut args),
        10 => day10::day10(&mut args),
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
