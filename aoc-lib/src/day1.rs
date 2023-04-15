use itertools::Itertools;

pub fn part1(data: &str) -> String {
    return parse_elves(data)
        .iter()
        .max()
        .expect("No groups found")
        .to_string();
}

pub fn part2(data: &str) -> String {
    let result: u64 = parse_elves(data).iter().sorted().rev().take(3).sum();
    return result.to_string();
}

fn parse_elves(data: &str) -> Vec<u64> {
    return data
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|item| item.parse::<u64>().expect("Invalid amount"))
                .sum()
        })
        .collect();
}

#[cfg(test)]
mod day1tests {
    use std::fs;

    use crate::day1;

    #[test]
    fn test_part1() {
        let data = fs::read_to_string("../data/1.in").unwrap();
        let result = day1::part1(&data);
        assert_eq!(result, "72478");
    }

    #[test]
    fn test_part2() {
        let data = fs::read_to_string("../data/1.in").unwrap();
        let result = day1::part2(&data);
        assert_eq!(result, "210367");
    }
}
