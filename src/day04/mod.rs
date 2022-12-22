use regex::Regex;

#[derive(Clone)]
struct Range {
    start: i32,
    end: i32,
}

struct Pair {
    first: Range,
    second: Range,
}

fn parse(input: &str) -> Vec<Pair> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Pair {
                first: Range {
                    start: caps[1].parse().unwrap(),
                    end: caps[2].parse().unwrap(),
                },
                second: Range {
                    start: caps[3].parse().unwrap(),
                    end: caps[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn count_pairs(pairs: Vec<Pair>, pred: fn(first: Range, second: Range) -> bool) -> i32 {
    pairs
        .into_iter()
        .filter(|Pair { first, second }| {
            pred(first.clone(), second.clone()) || pred(second.clone(), first.clone())
        })
        .count() as i32
}

fn contains(first: Range, second: Range) -> bool {
    first.start <= second.start && second.end <= first.end
}

pub fn puzzle1(input: &str) -> i32 {
    count_pairs(parse(input), contains)
}

fn left_overlap(first: Range, second: Range) -> bool {
    first.start <= second.start && second.start <= first.end
}

pub fn puzzle2(input: &str) -> i32 {
    count_pairs(parse(input), left_overlap)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 2);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 582);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 4);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 893);
    }
}
