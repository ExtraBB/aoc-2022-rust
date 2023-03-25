use std::fs;

pub mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("No day provided");
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
    match day {
        1 => {
            if part == 1 {
                day1::part1(data)
            } else if part == 2 {
                day1::part2(data)
            } else {
                panic!("Part does not exist")
            }
        }
        _ => panic!("Solution for day {day}, part {part} not implemented yet."),
    }
}
