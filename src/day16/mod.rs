use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use regex::Regex;

type Name = (char, char);

const START: Name = ('A', 'A');

fn parse(input: &str) -> Vec<(isize, Vec<isize>)> {
    let re =
        Regex::new(r"^Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)$").unwrap();
    let mut graph: HashMap<Name, (isize, HashMap<Name, isize>)> = input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                cap[1].chars().collect_tuple().unwrap(),
                (
                    cap[2].parse().unwrap(),
                    cap[3]
                        .split(", ")
                        .map(|name| (name.chars().collect_tuple().unwrap(), 1))
                        .collect(),
                ),
            )
        })
        .collect();
    assert_eq!(graph[&START].0, 0);
    for x in graph.keys().copied().collect::<Vec<_>>() {
        if let Some(((a, m), (b, n))) = {
            let (rate, tunnels) = &graph[&x];
            if x != START && *rate == 0 && tunnels.len() == 2 {
                tunnels.iter().map(|(&k, &v)| (k, v)).collect_tuple()
            } else {
                None
            }
        } {
            let (_, l) = &mut graph.get_mut(&a).unwrap();
            l.remove(&x);
            l.insert(b, m + n);
            let (_, r) = &mut graph.get_mut(&b).unwrap();
            r.remove(&x);
            r.insert(a, m + n);
            graph.remove(&x);
        }
    }
    let mut keys: Vec<Name> = graph.keys().copied().collect();
    keys.sort();
    let keymap: HashMap<Name, usize> = keys.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    keys.into_iter()
        .map(|x| {
            let mut times = vec![isize::MAX; keymap.len()];
            let mut queue = BinaryHeap::new();
            queue.push((0, x));
            while let Some((d, k)) = queue.pop() {
                let r = &mut times[keymap[&k]];
                if d <= -*r {
                    continue;
                }
                *r = -d;
                for (&y, &n) in &graph[&k].1 {
                    queue.push((d - n, y));
                }
            }
            (graph[&x].0, times)
        })
        .collect()
}

fn search(
    graph: &[(isize, Vec<isize>)],
    mask: usize,
    minute: isize,
    now: usize,
    current: isize,
) -> isize {
    let (rate, tunnels) = &graph[now];
    let flow: isize = current + rate;
    let mut pressure = flow * minute;
    for (i, v) in tunnels.iter().enumerate() {
        let bit = 1 << i;
        if bit & mask == 0 {
            let d = v + 1;
            let later = minute - d;
            if later > 0 {
                pressure = pressure.max(flow * d + search(graph, mask | bit, later, i, flow));
            }
        }
    }
    pressure
}

pub fn puzzle1(input: &str) -> isize {
    search(&parse(input), 1, 30, 0, 0)
}

pub fn puzzle2(input: &str) -> isize {
    let graph = parse(input);
    let best: Vec<_> = (0..1 << (graph.len() - 1))
        .map(|mask| search(&graph, (mask << 1) | 1, 26, 0, 0))
        .collect();
    let all = best.len() - 1;
    best.iter()
        .enumerate()
        .map(|(mask, total)| total + best[mask ^ all])
        .max()
        .unwrap()
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

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2967);
    }
}
