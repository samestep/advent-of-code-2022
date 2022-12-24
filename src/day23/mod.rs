use std::collections::{hash_map::Entry, HashMap, HashSet};

fn parse(input: &str) -> HashSet<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .collect()
}

enum Direction {
    N,
    S,
    W,
    E,
}

use Direction::*;

const START: [Direction; 4] = [N, S, W, E];

fn round(elves: &HashSet<(isize, isize)>, directions: &mut [Direction]) -> HashSet<(isize, isize)> {
    let mut proposed = HashMap::new();
    'elves: for &from in elves {
        let mut propose = |to| {
            if let Some(prev) = match proposed.entry(to) {
                Entry::Vacant(entry) => {
                    entry.insert(Some(from));
                    None
                }
                Entry::Occupied(mut entry) => Some(entry.insert(None)),
            } {
                proposed.insert(from, Some(from));
                if let Some(other) = prev {
                    proposed.insert(other, Some(other));
                }
            }
        };
        let (i, j) = from;
        let to_n = (i - 1, j);
        let to_s = (i + 1, j);
        let to_w = (i, j - 1);
        let to_e = (i, j + 1);
        let n = elves.contains(&to_n);
        let s = elves.contains(&to_s);
        let w = elves.contains(&to_w);
        let e = elves.contains(&to_e);
        let ne = elves.contains(&(i - 1, j + 1));
        let nw = elves.contains(&(i - 1, j - 1));
        let se = elves.contains(&(i + 1, j + 1));
        let sw = elves.contains(&(i + 1, j - 1));
        if !(n || s || w || e || ne || nw || se || sw) {
            propose(from);
            continue 'elves;
        }
        for direction in directions.iter() {
            match direction {
                N => {
                    if !(n || ne || nw) {
                        propose(to_n);
                        continue 'elves;
                    }
                }
                S => {
                    if !(s || se || sw) {
                        propose(to_s);
                        continue 'elves;
                    }
                }
                W => {
                    if !(w || nw || sw) {
                        propose(to_w);
                        continue 'elves;
                    }
                }
                E => {
                    if !(e || ne || se) {
                        propose(to_e);
                        continue 'elves;
                    }
                }
            }
        }
        propose(from);
    }
    directions.rotate_left(1);
    proposed
        .into_iter()
        .filter_map(|(to, from)| from.map(|_| to))
        .collect()
}

pub fn puzzle1(input: &str) -> isize {
    let mut elves = parse(input);
    let mut directions = START;
    for _ in 0..10 {
        elves = round(&elves, &mut directions);
    }
    let mut i_min = isize::MAX;
    let mut i_max = isize::MIN;
    let mut j_min = isize::MAX;
    let mut j_max = isize::MIN;
    for &(i, j) in &elves {
        i_min = i.min(i_min);
        i_max = i.max(i_max);
        j_min = j.min(j_min);
        j_max = j.max(j_max);
    }
    (1 + i_max - i_min) * (1 + j_max - j_min) - elves.len() as isize
}

pub fn puzzle2(input: &str) -> usize {
    let mut elves = parse(input);
    let mut directions = START;
    let mut n = 1;
    loop {
        let after = round(&elves, &mut directions);
        if after == elves {
            break;
        }
        elves = after;
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), 25);
    }

    #[test]
    fn test_puzzle1_example2() {
        assert_eq!(puzzle1(EXAMPLE2), 110);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 3800);
    }

    #[test]
    fn test_puzzle2_example1() {
        assert_eq!(puzzle2(EXAMPLE1), 4);
    }

    #[test]
    fn test_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), 20);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 916);
    }
}
