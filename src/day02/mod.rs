enum Left {
    A,
    B,
    C,
}

enum Right {
    X,
    Y,
    Z,
}

struct Strat {
    left: Left,
    right: Right,
}

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    opponent: Choice,
    me: Choice,
}

use Choice::*;
use Left::*;
use Right::*;

fn get_strats(input: &str) -> Vec<Strat> {
    input
        .lines()
        .map(|line| Strat {
            left: match &line[..1] {
                "A" => A,
                "B" => B,
                "C" => C,
                _ => panic!("invalid left"),
            },
            right: match &line[2..] {
                "X" => X,
                "Y" => Y,
                "Z" => Z,
                _ => panic!("invalid right"),
            },
        })
        .collect()
}

fn score_round(Round { opponent, me }: Round) -> i32 {
    (match me {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }) + (match (opponent, me) {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
    })
}

fn left_choice(left: Left) -> Choice {
    match left {
        A => Rock,
        B => Paper,
        C => Scissors,
    }
}

pub fn puzzle1(input: &str) -> i32 {
    get_strats(input)
        .into_iter()
        .map(|Strat { left, right }| {
            score_round(Round {
                opponent: left_choice(left),
                me: match right {
                    X => Rock,
                    Y => Paper,
                    Z => Scissors,
                },
            })
        })
        .sum()
}

pub fn puzzle2(input: &str) -> i32 {
    get_strats(input)
        .into_iter()
        .map(|Strat { left, right }| {
            let opponent = left_choice(left);
            score_round(Round {
                opponent,
                me: match (opponent, right) {
                    (Rock, X) => Scissors,
                    (Paper, X) => Rock,
                    (Scissors, X) => Paper,
                    (_, Y) => opponent,
                    (Rock, Z) => Paper,
                    (Paper, Z) => Scissors,
                    (Scissors, Z) => Rock,
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
        assert_eq!(puzzle1(EXAMPLE), 15);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 12458);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 12);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 12683);
    }
}
