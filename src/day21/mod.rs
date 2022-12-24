use std::collections::{HashMap, HashSet};

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

const HUMAN: &str = "humn";
const ROOT: &str = "root";

fn parse(input: &str) -> impl Iterator<Item = (&str, Job)> {
    input.lines().map(|line| {
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
}

fn get_parents<'a>(monkeys: &HashMap<&'a str, Job<'a>>) -> HashMap<&'a str, &'a str> {
    let mut parents = HashMap::<&str, &str>::new();
    for (name, job) in monkeys {
        if let Wait(left, _, right) = job {
            for other in [left, right] {
                assert_eq!(parents.insert(other, name), None);
            }
        }
    }
    parents
}

fn yell(monkeys: HashMap<&str, Job>, root: &str) -> isize {
    let parents = get_parents(&monkeys);
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
        if let Some(parent) = parents.get(name) {
            if let Wait(left, _, right) = monkeys[parent] {
                if yells.contains_key(left) && yells.contains_key(right) {
                    stack.push(parent);
                }
            }
        }
    }
    yells[root]
}

pub fn puzzle1(input: &str) -> isize {
    yell(parse(input).collect(), ROOT)
}

fn get_path(input: &str) -> HashSet<&str> {
    let mut path = HashSet::new();
    let monkeys: HashMap<&str, Job> = parse(input).collect();
    let parents = get_parents(&monkeys);
    let mut name = HUMAN;
    path.insert(name);
    while let Some(parent) = parents.get(name) {
        path.insert(parent);
        name = parent;
    }
    path
}

pub fn puzzle2(input: &str) -> isize {
    let path = get_path(input);
    yell(
        parse(input)
            .map(|(name, job)| {
                if name == HUMAN {
                    (ROOT, Lone(0))
                } else if path.contains(name) {
                    match job {
                        Wait(left, op, right) => {
                            let (n, l, o, r) = match (path.contains(left), op, path.contains(right))
                            {
                                (true, _, true) => unreachable!(),
                                (false, _, false) => unreachable!(),
                                (true, Add, false) => (left, name, Sub, right),
                                (false, Add, true) => (right, name, Sub, left),
                                (true, Sub, false) => (left, right, Add, name),
                                (false, Sub, true) => (right, left, Sub, name),
                                (true, Mul, false) => (left, name, Div, right),
                                (false, Mul, true) => (right, name, Div, left),
                                (true, Div, false) => (left, right, Mul, name),
                                (false, Div, true) => (right, left, Div, name),
                            };
                            (n, Wait(l, if name == ROOT { Add } else { o }, r))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    (name, job)
                }
            })
            .collect(),
        HUMAN,
    )
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

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 301);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 0);
    }
}
