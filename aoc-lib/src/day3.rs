pub fn part1(data: &str) -> String {
    return run(data, true);
}

fn run(data: &str, part1: bool) -> String {
    let sum: u32 = data
        .split("\n")
        .map(|line| char_to_priority(find_duplicate(line)))
        .sum();
    return sum.to_string();
}

fn find_duplicate(line: &str) -> char {
    let chars: Vec<char> = line.chars().collect();
    let half = &chars[..chars.len() / 2];
    for c in &chars[chars.len() / 2..] {
        if half.contains(c) {
            println!("{}", c);
            return *c;
        }
    }

    panic!("no duplicate in line");
}

fn char_to_priority(ch: char) -> u32 {
    return (if ch.is_uppercase() {
        ch as u8 - 'A' as u8 + 27
    } else {
        ch as u8 - 'a' as u8 + 1
    })
    .into();
}

#[cfg(test)]
mod day3tests {
    use std::fs;
    use test_case::test_case;

    use crate::day3;

    use super::char_to_priority;

    #[test_case('a', 1)]
    #[test_case('h', 8)]
    #[test_case('z', 26)]
    #[test_case('A', 27)]
    #[test_case('H', 34)]
    #[test_case('Z', 52)]
    fn test_char_to_priority(ch: char, result: u32) {
        assert_eq!(char_to_priority(ch), result);
    }

    #[test]
    fn test_part1() {
        let data = fs::read_to_string("../data/3.in").unwrap();
        let result = day3::part1(&data);
        assert_eq!(result, "7980");
    }

    // #[test]
    // fn test_part2() {
    //     let data = fs::read_to_string("../data/3.in").unwrap();
    //     let result = day3::part2(&data);
    //     assert_eq!(result, "13889");
    // }
}
