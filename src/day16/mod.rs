use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

type Name = (char, char);

struct Valve {
    rate: isize,
    tunnels: HashMap<Name, isize>,
}

const START: Name = ('A', 'A');

fn parse(input: &str) -> HashMap<Name, Valve> {
    let re =
        Regex::new(r"^Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)$").unwrap();
    let mut graph: HashMap<Name, Valve> = input
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
        .collect();
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
    graph
}

const LIMIT1: isize = 30;

fn search1(
    graph: &HashMap<Name, Valve>,
    minute: isize,
    now: Name,
    open: &mut HashSet<Name>,
) -> isize {
    let current: isize = open.iter().map(|x| graph.get(x).unwrap().rate).sum();
    let mut pressure = current * (LIMIT1 - minute);
    for (&k, &v) in &graph.get(&now).unwrap().tunnels {
        if !open.contains(&k) {
            let d = v + 1;
            let later = minute + d;
            if later < LIMIT1 {
                open.insert(k);
                pressure = pressure.max(current * d + search1(graph, later, k, open));
                open.remove(&k);
            }
        }
    }
    pressure
}

pub fn puzzle1(input: &str) -> isize {
    search1(&parse(input), 0, START, &mut HashSet::new())
}

fn search2(
    graph: &HashMap<Name, Valve>,
    minute: isize,
    me: (isize, Name),
    elephant: (isize, Name),
    open: &mut HashSet<Name>,
) -> isize {
    if minute == 0 {
        return 0;
    }
    if me.0 == 0 {
        let mut pressure = None;
        for (&k, &v) in &graph.get(&me.1).unwrap().tunnels {
            let d = v + 1;
            if !open.contains(&k) && d < minute {
                open.insert(k);
                pressure =
                    Some(
                        pressure
                            .unwrap_or(0)
                            .max(search2(graph, minute, (d, k), elephant, open)),
                    );
                open.remove(&k);
            }
        }
        if let Some(n) = pressure {
            return n;
        }
    }
    if elephant.0 == 0 {
        let mut pressure = None;
        for (&k, &v) in &graph.get(&elephant.1).unwrap().tunnels {
            let d = v + 1;
            if !open.contains(&k) && d < minute {
                open.insert(k);
                pressure =
                    Some(
                        pressure
                            .unwrap_or(0)
                            .max(search2(graph, minute, me, (d, k), open)),
                    );
                open.remove(&k);
            }
        }
        if let Some(n) = pressure {
            return n;
        }
    }
    let current: isize = open
        .iter()
        .filter_map(|&x| {
            if (x != me.1 && x != elephant.1)
                || (x == me.1 && me.0 == 0)
                || (x == elephant.1 && elephant.0 == 0)
            {
                Some(graph.get(&x).unwrap().rate)
            } else {
                None
            }
        })
        .sum();
    let mut t = minute;
    if me.0 > 0 {
        t = t.min(me.0);
    }
    if elephant.0 > 0 {
        t = t.min(elephant.0);
    }
    current * t
        + search2(
            graph,
            minute - t,
            ((me.0 - t).max(0), me.1),
            ((elephant.0 - t).max(0), elephant.1),
            open,
        )
}

pub fn puzzle2(input: &str) -> isize {
    let mut open = HashSet::new();
    open.insert(START);
    search2(&parse(input), 26, (0, START), (0, START), &mut open)
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

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 1707);
    }

    #[ignore]
    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2967);
    }
}
