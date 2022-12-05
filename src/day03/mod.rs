use std::collections::HashSet;

use itertools::Itertools;

fn charset(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn prioritize(c: char) -> Option<i32> {
    match c {
        'a'..='z' => Some(1 + (c as i32 - 'a' as i32)),
        'A'..='Z' => Some(27 + (c as i32 - 'A' as i32)),
        _ => None,
    }
}

pub fn puzzle1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let n = line.len() / 2;
            charset(&line[..n])
                .intersection(&charset(&line[n..]))
                .map(|c| prioritize(*c).unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

pub fn puzzle2(input: &str) -> String {
    let mut sum = 0;
    for mut group in &input.lines().chunks(3) {
        let first = group.next().unwrap();
        let mut chars = charset(first);
        for line in group {
            chars = chars.intersection(&charset(line)).copied().collect();
        }
        sum += chars
            .into_iter()
            .map(|c| prioritize(c).unwrap())
            .sum::<i32>();
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "157");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "7826");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "70");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "2577");
    }
}
