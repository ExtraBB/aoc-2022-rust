pub fn part1(data: &str) {
    let max: u64 = data
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|item| item.parse::<u64>().expect("Invalid amount"))
                .sum()
        })
        .max()
        .expect("No groups found");

    println!("Result: {max}");
}

pub fn part2(_data: &str) {
    unimplemented!();
}
