use regex::Regex;

const N: usize = 4;

type Vector = [usize; N];
type Matrix = [Vector; N];

fn parse(input: &str) -> Vec<Matrix> {
    let re = Regex::new(concat!(
        r"(?m)Blueprint \d+:",
        r"\s+Each ore robot costs (\d+) ore.",
        r"\s+Each clay robot costs (\d+) ore.",
        r"\s+Each obsidian robot costs (\d+) ore and (\d+) clay.",
        r"\s+Each geode robot costs (\d+) ore and (\d+) obsidian.",
    ))
    .unwrap();
    re.captures_iter(input)
        .map(|caps| {
            [
                [caps[1].parse().unwrap(), 0, 0, 0],
                [caps[2].parse().unwrap(), 0, 0, 0],
                [caps[3].parse().unwrap(), caps[4].parse().unwrap(), 0, 0],
                [caps[5].parse().unwrap(), 0, caps[6].parse().unwrap(), 0],
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

fn best(m: &Matrix, s: State, a: usize, b: bool) -> usize {
    let mut geodes = s.resources[N - 1] + s.minutes * s.robots[N - 1];
    if s.minutes == 0 {
        return geodes;
    }
    'outer: for i in a.saturating_sub(1)..N.min(a + 2) {
        if i < a && b {
            continue;
        }
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
            geodes = geodes.max(best(
                m,
                State {
                    minutes,
                    robots,
                    resources,
                },
                i,
                i < a || (i == a && b),
            ));
        }
    }
    geodes
}

pub fn puzzle1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            (i + 1)
                * best(
                    &blueprint,
                    State {
                        minutes: 24,
                        robots: [1, 0, 0, 0],
                        resources: [0; N],
                    },
                    0,
                    false,
                )
        })
        .sum()
}

pub fn puzzle2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .take(3)
        .map(|blueprint| {
            best(
                &blueprint,
                State {
                    minutes: 32,
                    robots: [1, 0, 0, 0],
                    resources: [0; N],
                },
                0,
                false,
            )
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
