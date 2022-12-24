use std::collections::HashMap;

use itertools::Itertools;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Job<'a> {
    Lone(isize),
    Wait(&'a str, Op, &'a str),
}

use Job::*;
use Op::*;

fn parse(input: &str) -> HashMap<&str, Job> {
    input
        .lines()
        .map(|line| {
            let (name, job) = line.split(": ").collect_tuple().unwrap();
            (
                name,
                if let Ok(n) = job.parse() {
                    Lone(n)
                } else {
                    let (left, op, right) = job.split(' ').collect_tuple().unwrap();
                    Wait(
                        left,
                        match op {
                            "+" => Add,
                            "-" => Sub,
                            "*" => Mul,
                            "/" => Div,
                            _ => panic!(),
                        },
                        right,
                    )
                },
            )
        })
        .collect()
}

pub fn puzzle1(input: &str) -> isize {
    let monkeys = parse(input);
    let mut edges = HashMap::<&str, Vec<&str>>::new();
    for (name, job) in &monkeys {
        if let Wait(left, _, right) = job {
            for other in [left, right] {
                edges.entry(other).or_insert_with(Vec::new).push(name);
            }
        }
    }
    let mut yells = HashMap::<&str, isize>::new();
    let mut stack = monkeys
        .iter()
        .filter_map(|(&name, job)| match job {
            Lone(_) => Some(name),
            _ => None,
        })
        .collect::<Vec<&str>>();
    while let Some(name) = stack.pop() {
        match &monkeys[name] {
            &Lone(n) => {
                yells.insert(name, n);
            }
            Wait(left, op, right) => {
                let x = yells[left];
                let y = yells[right];
                yells.insert(
                    name,
                    match op {
                        Add => x + y,
                        Sub => x - y,
                        Mul => x * y,
                        Div => x / y,
                    },
                );
            }
        }
        if let Some(others) = edges.get(name) {
            for other in others {
                if let Wait(left, _, right) = monkeys[other] {
                    if yells.contains_key(left) && yells.contains_key(right) {
                        stack.push(other);
                    }
                }
            }
        }
    }
    yells["root"]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 152);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 31017034894002);
    }
}
