use std::mem;

use regex::Regex;

enum Operator {
    Plus,
    Times,
}

use Operator::*;

struct Operation {
    left: Option<usize>,
    operator: Operator,
    right: Option<usize>,
}

impl Operation {
    fn eval(&self, x: usize) -> usize {
        let left = self.left.unwrap_or(x);
        let right = self.right.unwrap_or(x);
        match self.operator {
            Plus => left + right,
            Times => left * right,
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
}

fn parse_operand(s: &str) -> Option<usize> {
    match s {
        "old" => None,
        _ => Some(s.parse().unwrap()),
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let lines = input.lines().collect::<Vec<_>>();
    let n = lines.len() + 1;
    assert_eq!(n % 7, 0);

    let re_items = Regex::new(r"^  Starting items: (.+)$").unwrap();
    let re_operation = Regex::new(r"^  Operation: new = (.+) (.) (.+)$").unwrap();
    let re_test = Regex::new(r"^  Test: divisible by (.+)$").unwrap();
    let re_if = Regex::new(r"^    If .+: throw to monkey (.+)$").unwrap();

    (0..n / 7)
        .map(|i| {
            let j = i * 7;
            Monkey {
                items: re_items.captures(lines[j + 1]).unwrap()[1]
                    .split(", ")
                    .map(|s| s.parse().unwrap())
                    .collect(),
                operation: {
                    let caps = re_operation.captures(lines[j + 2]).unwrap();
                    Operation {
                        left: parse_operand(&caps[1]),
                        operator: match &caps[2] {
                            "+" => Operator::Plus,
                            "*" => Operator::Times,
                            _ => panic!("unknown operator"),
                        },
                        right: parse_operand(&caps[3]),
                    }
                },
                test: re_test.captures(lines[j + 3]).unwrap()[1].parse().unwrap(),
                if_true: re_if.captures(lines[j + 4]).unwrap()[1].parse().unwrap(),
                if_false: re_if.captures(lines[j + 5]).unwrap()[1].parse().unwrap(),
            }
        })
        .collect()
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, f: impl Fn(usize) -> usize) -> isize {
    let mut inspections = vec![0isize; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in mem::take(&mut monkeys[i].items) {
                inspections[i] += 1;
                let worry = f(monkeys[i].operation.eval(item));
                let j = if worry % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[j].items.push(worry);
            }
        }
    }
    inspections.sort_by_key(|x| -x);
    inspections[0] * inspections[1]
}

pub fn puzzle1(input: &str) -> isize {
    solve(parse(input), 20, |x| x / 3)
}

pub fn puzzle2(input: &str) -> isize {
    let monkeys = parse(input);
    let n: usize = monkeys.iter().map(|monkey| monkey.test).product();
    solve(monkeys, 10000, |x| x % n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 10605);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 50616);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 2713310158);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 11309046332);
    }
}
