pub fn part1(data: &str) -> String {
    let lines = data.split("\n");
    let sum: u32 = lines.map(get_score).sum();

    return sum.to_string();
}

pub fn part2(data: &str) -> String {
    let lines = data.split("\n");
    let sum: u32 = lines.map(get_score_2).sum();
    return sum.to_string();
}

fn get_score(line: &str) -> u32 {
    match line {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

fn get_score_2(line: &str) -> u32 {
    match line {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}
