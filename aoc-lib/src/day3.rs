use std::collections::{HashMap, HashSet};

pub fn part1(data: &str) -> String {
    let sum: u32 = data
        .split("\n")
        .map(|line| {
            char_to_priority(find_duplicate(vec![
                &line[..line.len() / 2],
                &line[line.len() / 2..],
            ]))
        })
        .sum();
    return sum.to_string();
}

pub fn part2(data: &str) -> String {
    let lines: Vec<&str> = data.split("\n").collect();
    let mut sum = 0;
    for i in 0..lines.len() / 3 {
        sum += char_to_priority(find_duplicate(vec![
            lines[i * 3],
            lines[i * 3 + 1],
            lines[i * 3 + 2],
        ]));
    }

    return sum.to_string();
}

fn find_duplicate(sets: Vec<&str>) -> char {
    let mut map = HashMap::new();

    for c in sets[0].chars() {
        map.insert(c, 1);
    }

    for i in 1..sets.len() - 1 {
        let mut seen = HashSet::new();
        for c in sets[i].chars() {
            if seen.contains(&c) {
                continue;
            }
            seen.insert(c);
            if map.contains_key(&c) {
                map.insert(c, map[&c] + 1);
            }
        }
    }

    for c in sets[sets.len() - 1].chars() {
        if map.contains_key(&c) && map[&c] == sets.len() - 1 {
            return c;
        }
    }

    panic!("no duplicate")
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

    #[test]
    fn test_part2() {
        let data = fs::read_to_string("../data/3.in").unwrap();
        let result = day3::part2(&data);
        assert_eq!(result, "2881");
    }
}
