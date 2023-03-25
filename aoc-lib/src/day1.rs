use itertools::Itertools;

pub fn part1(data: &str) -> String {
    return parse_elves(data)
        .max()
        .expect("No groups found")
        .to_string();
}

pub fn part2(data: &str) -> String {
    let result: u64 = parse_elves(data).sorted().rev().take(3).sum();
    return result.to_string();
}

fn parse_elves(data: &str) -> impl Iterator<Item = u64> + '_ {
    return data.split("\n\n").map(|group| {
        group
            .split("\n")
            .map(|item| item.parse::<u64>().expect("Invalid amount"))
            .sum()
    });
}
