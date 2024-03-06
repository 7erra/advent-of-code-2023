use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(path: &str) -> u64 {
    let input = parse_input(path);
    get_min_pos(&input)
}

fn get_min_pos(input: &Input) -> u64 {
    let mut keys = input.seeds.clone();
    for k in keys.iter_mut() {
        for map in &input.maps {
            'maploop: for range in map {
                let (dest, source, delta) = range;
                if *k >= *source && *k < *source + *delta {
                    *k = *dest + (*k - *source);
                    break 'maploop;
                }
            }
        }
    }
    *keys.iter().min().unwrap()
}

#[derive(Default, Debug)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

fn parse_input(path: &str) -> Input {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut input = Input::default();
    for line in reader.lines() {
        match line.unwrap().as_str() {
            // Seeds (first line)
            l if l.contains("seeds") => input.seeds = parse_numbers_from_line(l.to_string()),
            // One of the maps
            l if l.contains("map") => input.maps.push(vec![]),
            // Empty line
            "" => (),
            // Numbers
            l => {
                if let [start, to, range] = parse_numbers_from_line(l.to_string()).as_slice() {
                    input.maps.last_mut().unwrap().push((*start, *to, *range));
                }
            }
        }
    }
    input
}

fn parse_numbers_from_line(line: String) -> Vec<u64> {
    let mut numbers: Vec<u64> = vec![];
    for x in line.split(' ') {
        if let Ok(n) = x.parse() {
            numbers.push(n);
        }
    }
    numbers
}

pub fn part2(path: &str) -> u64 {
    let mut input = parse_input(path);
    input.seeds = input
        .seeds
        .chunks(2)
        .flat_map(|c| {
            if let [x, y] = c {
                (*x..*x + *y).collect()
            } else {
                vec![]
            }
        })
        .collect();
    get_min_pos(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        assert_eq!(
            parse_numbers_from_line("seeds: 79 14 55 13".to_string()),
            vec![79, 14, 55, 13]
        );
    }

    #[test]
    fn test_part1_example() {
        let actual = part1("./data/05/example.txt");
        let expected = 35;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_input() {
        let actual = part1("./data/05/input.txt");
        let expected = 910845529;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_get_min_pos() {
        let example_input = parse_input("./data/05/example.txt");
        let input = Input {
            seeds: vec![82],
            ..example_input
        };
        let actual = get_min_pos(&input);
        let expected = 46;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_example() {
        let actual = part2("./data/05/example.txt");
        let expected = 46;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2("./data/05/input.txt"), 0);
    }
}
