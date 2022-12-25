use regex::Regex;

const N: usize = 4;

type Vector = [usize; N];
type Matrix = [Vector; N];

fn parse(input: &str) -> Vec<Matrix> {
    Regex::new(concat!(
        r"(?m)Blueprint \d+:",
        r"\s+Each ore robot costs (\d+) ore.",
        r"\s+Each clay robot costs (\d+) ore.",
        r"\s+Each obsidian robot costs (\d+) ore and (\d+) clay.",
        r"\s+Each geode robot costs (\d+) ore and (\d+) obsidian.",
    ))
    .unwrap()
    .captures_iter(input)
    .map(|cap| {
        [
            [cap[1].parse().unwrap(), 0, 0, 0],
            [cap[2].parse().unwrap(), 0, 0, 0],
            [cap[3].parse().unwrap(), cap[4].parse().unwrap(), 0, 0],
            [cap[5].parse().unwrap(), 0, cap[6].parse().unwrap(), 0],
        ]
    })
    .collect()
}

struct State {
    minutes: usize,
    robots: Vector,
    resources: Vector,
}

fn div_ceil(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}

fn search(m: &Matrix, geodes: &mut usize, s: State) {
    let baseline = s.resources[N - 1] + s.minutes * s.robots[N - 1];
    *geodes = (*geodes).max(baseline);
    if s.minutes == 0 || baseline + (s.minutes - 1) * s.minutes / 2 <= *geodes {
        return;
    }
    'outer: for i in (0..N).rev() {
        let mut wait = 0;
        for j in 0..N {
            let c = m[i][j];
            if c > 0 {
                let r = s.robots[j];
                if r == 0 {
                    continue 'outer;
                }
                wait = wait.max(div_ceil(c.saturating_sub(s.resources[j]), r));
            }
        }
        if let Some(minutes) = s.minutes.checked_sub(wait + 1) {
            let mut robots = s.robots;
            robots[i] += 1;
            let mut resources = s.resources;
            for j in 0..N {
                resources[j] += (s.minutes - minutes) * s.robots[j];
                resources[j] -= m[i][j];
            }
            search(
                m,
                geodes,
                State {
                    minutes,
                    robots,
                    resources,
                },
            );
        }
    }
}

pub fn puzzle1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let mut geodes = 0;
            search(
                &blueprint,
                &mut geodes,
                State {
                    minutes: 24,
                    robots: [1, 0, 0, 0],
                    resources: [0; N],
                },
            );
            (i + 1) * geodes
        })
        .sum()
}

pub fn puzzle2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .take(3)
        .map(|blueprint| {
            let mut geodes = 0;
            search(
                &blueprint,
                &mut geodes,
                State {
                    minutes: 32,
                    robots: [1, 0, 0, 0],
                    resources: [0; N],
                },
            );
            geodes
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 33);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1389);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 56 * 62);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 3003);
    }
}
