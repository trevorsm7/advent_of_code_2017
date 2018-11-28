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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        11 => day11::day11(&mut args),
        12 => day12::day12(&mut args),
        13 => day13::day13(&mut args),
        14 => day14::day14(&mut args),
        15 => day15::day15(&mut args),
        16 => day16::day16(&mut args),
        17 => day17::day17(&mut args),
        18 => day18::day18(&mut args),
        19 => day19::day19(&mut args),
        20 => day20::day20(&mut args),
        21 => day21::day21(&mut args),
        22 => day22::day22(&mut args),
        23 => day23::day23(&mut args),
        24 => day24::day24(&mut args),
        25 => day25::day25(&mut args),
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
