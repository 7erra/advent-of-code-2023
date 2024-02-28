use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fold_input<F>(path: &str, f_value: F) -> i32
where
    F: Fn(String) -> i32,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().fold(0, |acc, l| acc + f_value(l.unwrap()))
}

pub fn part1(path: &str) -> i32 {
    fold_input(path, card_value)
}

fn card_value(line: String) -> i32 {
    if let [wins, got] = strip_cardnumber(line)
        .split('|')
        .map(extract_ints)
        .collect::<Vec<HashSet<i32>>>()
        .as_slice()
    {
        match wins.intersection(got).count() {
            0 => 0,
            n => 2i32.pow((n - 1) as u32),
        }
    } else {
        dbg!("Failed to parse");
        0
    }
}

fn strip_cardnumber(card: String) -> String {
    card[(card.find(':').unwrap() + 1)..].to_string()
}

fn extract_ints(text: &str) -> HashSet<i32> {
    // Define the regex pattern to match integers
    let re = Regex::new(r"\b\d+\b").unwrap();

    // Initialize a vector to store the integers
    let mut integers: HashSet<i32> = HashSet::new();

    // Find all matches in the text and parse them into integers
    for mat in re.find_iter(text) {
        if let Ok(num) = mat.as_str().parse::<i32>() {
            integers.insert(num);
        }
    }
    integers
}

pub fn part2(path: &str) -> i32 {
    fold_input(path, card_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_line1() {
        assert_eq!(
            card_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()),
            8
        )
    }
    #[test]
    fn test_part1_example() {
        assert_eq!(part1("./data/04/example.txt"), 13);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(part1("./data/04/input.txt"), 26914);
    }
}
