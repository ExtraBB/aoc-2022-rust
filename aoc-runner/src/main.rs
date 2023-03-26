use aoc_lib::{day1, day2};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No day provided");
    }

    if args.len() < 3 {
        panic!("No part provided");
    }

    let day: u8 = match args[1].trim().parse() {
        Ok(day) => day,
        Err(_) => panic!("Invalid day provided"),
    };

    let part: u8 = match args[2].trim().parse() {
        Ok(part) => part,
        Err(_) => panic!("Invalid part provided"),
    };

    let path = format!("data/{}.in", args[1]);
    let data = fs::read_to_string(path).expect("No data found for this day");
    execute(day, part, &data);
}

// TODO: make Day trait
fn execute(day: u8, part: u8, data: &str) {
    let result = match (day, part) {
        (1, 1) => day1::part1(data),
        (1, 2) => day1::part2(data),
        (2, 1) => day2::part1(data),
        (2, 2) => day2::part2(data),
        _ => panic!("Solution for day {day}, part {part} not implemented yet."),
    };

    println!("Result: {result}");
}
