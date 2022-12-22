use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

type Name = (char, char);

#[derive(Debug)]
struct Valve {
    rate: isize,
    tunnels: HashMap<Name, isize>,
}

fn parse(input: &str) -> HashMap<Name, Valve> {
    let re =
        Regex::new(r"^Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].chars().collect_tuple().unwrap(),
                Valve {
                    rate: caps[2].parse().unwrap(),
                    tunnels: caps[3]
                        .split(", ")
                        .map(|name| (name.chars().collect_tuple().unwrap(), 1))
                        .collect(),
                },
            )
        })
        .collect()
}

const LIMIT: isize = 30;
const START: Name = ('A', 'A');

fn search(
    graph: &HashMap<Name, Valve>,
    minute: isize,
    now: Name,
    open: &mut HashSet<Name>,
) -> isize {
    let current: isize = open.iter().map(|x| graph.get(x).unwrap().rate).sum();
    let mut pressure = current * (LIMIT - minute);
    for (&k, &v) in &graph.get(&now).unwrap().tunnels {
        if !open.contains(&k) {
            let d = v + 1;
            let later = minute + d;
            if later < LIMIT {
                open.insert(k);
                pressure = pressure.max(current * d + search(graph, later, k, open));
                open.remove(&k);
            }
        }
    }
    pressure
}

pub fn puzzle1(input: &str) -> isize {
    let mut graph = parse(input);
    assert_eq!(graph.get(&START).unwrap().rate, 0);
    for x in graph.keys().copied().collect::<Vec<_>>() {
        if let Some(((a, m), (b, n))) = {
            let Valve { rate, tunnels, .. } = graph.get(&x).unwrap();
            if x != START && *rate == 0 && tunnels.len() == 2 {
                tunnels.iter().map(|(&k, &v)| (k, v)).collect_tuple()
            } else {
                None
            }
        } {
            let l = &mut graph.get_mut(&a).unwrap().tunnels;
            l.remove(&x);
            l.insert(b, m + n);
            let r = &mut graph.get_mut(&b).unwrap().tunnels;
            r.remove(&x);
            r.insert(a, m + n);
            graph.remove(&x);
        }
    }
    for x in graph.keys().copied().collect::<Vec<_>>() {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push((0, x));
        while let Some((d, k)) = queue.pop() {
            if visited.contains(&k) {
                continue;
            }
            visited.insert(k);
            graph.get_mut(&x).unwrap().tunnels.entry(k).or_insert(-d);
            let Valve { tunnels, .. } = graph.get_mut(&k).unwrap();
            tunnels.entry(x).or_insert(-d);
            for (&y, &mut n) in tunnels {
                queue.push((d - n, y));
            }
        }
    }
    search(&graph, 0, START, &mut HashSet::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1651);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 2320);
    }
}
