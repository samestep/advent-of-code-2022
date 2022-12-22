use std::collections::{HashMap, HashSet};

use regex::Regex;

const X_SAND: usize = 500;
const Y_SAND: usize = 0;

fn parse(input: &str) -> HashMap<(usize, usize), char> {
    let re = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let mut cave = HashMap::new();
    cave.insert((X_SAND, Y_SAND), '+');
    for line in input.lines() {
        let mut it = line.split(" -> ").map(|point| {
            let caps = re.captures(point).unwrap();
            (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            )
        });
        let (mut x0, mut y0) = it.next().unwrap();
        for (x1, y1) in it {
            for x in x0.min(x1)..=x0.max(x1) {
                for y in y0.min(y1)..=y0.max(y1) {
                    cave.insert((x, y), '#');
                }
            }
            (x0, y0) = (x1, y1);
        }
    }
    cave
}

pub fn puzzle1(input: &str) -> String {
    let mut cave = parse(input);
    let y_max = cave.keys().copied().map(|(_, y)| y).max().unwrap();
    let mut path = HashSet::new();
    loop {
        let mut falling = HashSet::new();
        let (mut x, mut y) = (X_SAND, Y_SAND);
        while y < y_max {
            falling.insert((x, y));
            if !cave.contains_key(&(x, y + 1)) {
                y += 1;
            } else if !cave.contains_key(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !cave.contains_key(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                cave.insert((x, y), 'o');
                break;
            }
        }
        if falling == path {
            break;
        }
        path = falling;
    }
    cave.values().filter(|&&c| c == 'o').count().to_string()
}

pub fn puzzle2(input: &str) -> String {
    let mut cave = parse(input);
    let y_max = cave.keys().copied().map(|(_, y)| y).max().unwrap();
    while cave.get(&(X_SAND, Y_SAND)) == Some(&'+') {
        let (mut x, mut y) = (X_SAND, Y_SAND);
        loop {
            if y < y_max + 1 && !cave.contains_key(&(x, y + 1)) {
                y += 1;
            } else if y < y_max + 1 && !cave.contains_key(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if y < y_max + 1 && !cave.contains_key(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                cave.insert((x, y), 'o');
                break;
            }
        }
    }
    cave.values().filter(|&&c| c == 'o').count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "24");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "825");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "93");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "26729");
    }
}
