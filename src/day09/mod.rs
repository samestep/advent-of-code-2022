use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

use Dir::*;

fn parse(input: &str) -> impl Iterator<Item = (Dir, i32)> + '_ {
    input.lines().map(|line| {
        let (d, n) = line.split_whitespace().collect_tuple().unwrap();
        (
            match d {
                "U" => U,
                "D" => D,
                "L" => L,
                "R" => R,
                _ => panic!(),
            },
            n.parse().unwrap(),
        )
    })
}

fn motion(d: Dir, h: &mut Pos) {
    match d {
        U => h.y += 1,
        D => h.y -= 1,
        L => h.x -= 1,
        R => h.x += 1,
    }
}

fn follow(h: Pos, t: &mut Pos) {
    if !((h.x - t.x).abs() <= 1 && (h.y - t.y).abs() <= 1) {
        if h.x < t.x {
            t.x -= 1;
        } else if t.x < h.x {
            t.x += 1;
        }
        if h.y < t.y {
            t.y -= 1;
        } else if t.y < h.y {
            t.y += 1;
        }
    }
}

fn solve<const L: usize>(input: &str) -> String {
    let mut positions = HashSet::new();
    let mut rope = [Pos { x: 0, y: 0 }; L];
    for (d, n) in parse(input) {
        for _ in 0..n {
            motion(d, &mut rope[0]);
            for i in 1..L {
                follow(rope[i - 1], &mut rope[i]);
            }
            positions.insert(rope[L - 1]);
        }
    }
    positions.len().to_string()
}

pub fn puzzle1(input: &str) -> String {
    solve::<2>(input)
}

pub fn puzzle2(input: &str) -> String {
    solve::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), "13");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "6339");
    }

    #[test]
    fn test_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), "36");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "2541");
    }
}
