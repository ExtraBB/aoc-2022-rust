pub fn part1(data: &str) -> String {
    return run(data, true);
}

pub fn part2(data: &str) -> String {
    return run(data, false);
}

fn run(data: &str, part1: bool) -> String {
    let sum: u32 = data.split("\n").map(|line| get_score(line, part1)).sum();
    return sum.to_string();
}

fn get_score(line: &str, part1: bool) -> u32 {
    let scores = if part1 {
        [4, 8, 3, 1, 5, 9, 7, 2, 6]
    } else {
        [3, 4, 8, 1, 5, 9, 2, 6, 7]
    };

    let first = line.chars().next().unwrap();
    let last = line.chars().last().unwrap();

    let first = first as u8 - 'A' as u8;
    let last = last as u8 - 'X' as u8;
    return scores[(first * 3 + last) as usize];
}

#[cfg(test)]
mod day2tests {
    use std::fs;

    use crate::day2;

    #[test]
    fn test_part1() {
        let data = fs::read_to_string("../data/2.in").unwrap();
        let result = day2::part1(&data);
        assert_eq!(result, "14827");
    }

    #[test]
    fn test_part2() {
        let data = fs::read_to_string("../data/2.in").unwrap();
        let result = day2::part2(&data);
        assert_eq!(result, "13889");
    }
}
