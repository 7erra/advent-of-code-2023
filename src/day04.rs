use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// fn fold_input<F>(path: &str, f_value: F) -> i32
fn input_lines(path: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
    // reader.lines().fold(0, |acc, l| acc + f_value(l.unwrap()))
}

pub fn part1(path: &str) -> i32 {
    input_lines(path).fold(0, |acc, l| acc + card_value(l.unwrap()))
}

fn win_count(wins: &HashSet<i32>, got: &HashSet<i32>) -> i32 {
    wins.intersection(got).count() as i32
}

fn card_value(line: String) -> i32 {
    let (wins, gots) = get_wins_and_gots(line);
    match win_count(&wins, &gots) {
        0 => 0,
        n => 2i32.pow((n - 1) as u32),
    }
}

fn strip_cardnumber(card: String) -> String {
    card[(card.find(':').unwrap() + 1)..].to_string()
}

fn get_wins_and_gots(card: String) -> (HashSet<i32>, HashSet<i32>) {
    if let [wins, gots] = strip_cardnumber(card)
        .split('|')
        .map(extract_ints)
        .collect::<Vec<HashSet<i32>>>()
        .as_slice()
    {
        (wins.clone(), gots.clone())
    } else {
        (HashSet::new(), HashSet::new())
    }
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
    let mut count = 0;
    let mut deque: VecDeque<i32> = VecDeque::from([1]);
    for line in input_lines(path) {
        // The first element in the deque is the current card
        let card_multiplier = deque.pop_front().unwrap();
        count += card_multiplier;

        let l = line.unwrap();
        let (wins, gots) = get_wins_and_gots(l);
        let winners = win_count(&wins, &gots);
        let mut win_iter = winners;
        for x in deque.iter_mut() {
            if win_iter == 0 {
                break;
            }
            win_iter -= 1;
            *x += card_multiplier;
        }
        for _ in 0..win_iter {
            deque.push_back(1 + card_multiplier);
        }
        if deque.is_empty() {
            deque.push_back(1)
        }
    }
    count
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

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("./data/04/example.txt"), 30);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2("./data/04/input.txt"), 13080971);
    }
}
