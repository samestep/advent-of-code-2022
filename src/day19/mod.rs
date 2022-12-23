use std::ops::{Add, Mul};

use regex::Regex;

struct Blueprint {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

fn parse(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(concat!(
        r"(?m)Blueprint \d+:",
        r"\s+Each ore robot costs (\d+) ore.",
        r"\s+Each clay robot costs (\d+) ore.",
        r"\s+Each obsidian robot costs (\d+) ore and (\d+) clay.",
        r"\s+Each geode robot costs (\d+) ore and (\d+) obsidian.",
    ))
    .unwrap();
    re.captures_iter(input)
        .map(|caps| Blueprint {
            ore: caps[1].parse().unwrap(),
            clay: caps[2].parse().unwrap(),
            obsidian: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            geode: (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Have {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Mul<Have> for usize {
    type Output = Have;

    fn mul(self, rhs: Have) -> Self::Output {
        Have {
            ore: self * rhs.ore,
            clay: self * rhs.clay,
            obsidian: self * rhs.obsidian,
            geode: self * rhs.geode,
        }
    }
}

impl Add for Have {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Have {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

struct State {
    minutes: usize,
    robots: Have,
    resources: Have,
}

fn div_ceil(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}

impl Blueprint {
    fn best(&self, s: State) -> usize {
        let mut geodes = s.resources.geode + s.minutes * s.robots.geode;
        if s.minutes == 0 {
            return geodes;
        }
        if let Some(minutes) = s
            .minutes
            .checked_sub(1 + div_ceil(self.ore.saturating_sub(s.resources.ore), s.robots.ore))
        {
            let mut robots = s.robots;
            robots.ore += 1;
            let mut resources = s.resources + (s.minutes - minutes) * s.robots;
            resources.ore -= self.ore;
            geodes = geodes.max(self.best(State {
                minutes,
                robots,
                resources,
            }));
        }
        if let Some(minutes) = s
            .minutes
            .checked_sub(1 + div_ceil(self.clay.saturating_sub(s.resources.ore), s.robots.ore))
        {
            let mut robots = s.robots;
            robots.clay += 1;
            let mut resources = s.resources + (s.minutes - minutes) * s.robots;
            resources.ore -= self.clay;
            geodes = geodes.max(self.best(State {
                minutes,
                robots,
                resources,
            }));
        }
        if s.robots.clay > 0 {
            if let Some(minutes) = s.minutes.checked_sub(
                1 + div_ceil(
                    self.obsidian.0.saturating_sub(s.resources.ore),
                    s.robots.ore,
                )
                .max(div_ceil(
                    self.obsidian.1.saturating_sub(s.resources.clay),
                    s.robots.clay,
                )),
            ) {
                let mut robots = s.robots;
                robots.obsidian += 1;
                let mut resources = s.resources + (s.minutes - minutes) * s.robots;
                resources.ore -= self.obsidian.0;
                resources.clay -= self.obsidian.1;
                geodes = geodes.max(self.best(State {
                    minutes,
                    robots,
                    resources,
                }));
            }
        }
        if s.robots.obsidian > 0 {
            if let Some(minutes) = s.minutes.checked_sub(
                1 + div_ceil(self.geode.0.saturating_sub(s.resources.ore), s.robots.ore).max(
                    div_ceil(
                        self.geode.1.saturating_sub(s.resources.obsidian),
                        s.robots.obsidian,
                    ),
                ),
            ) {
                let mut robots = s.robots;
                robots.geode += 1;
                let mut resources = s.resources + (s.minutes - minutes) * s.robots;
                resources.ore -= self.geode.0;
                resources.obsidian -= self.geode.1;
                geodes = geodes.max(self.best(State {
                    minutes,
                    robots,
                    resources,
                }));
            }
        }
        geodes
    }
}

pub fn puzzle1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            (i + 1)
                * blueprint.best(State {
                    minutes: 24,
                    robots: Have {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    resources: Have {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                })
        })
        .sum()
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
}
