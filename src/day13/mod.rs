use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

use Packet::*;

fn parse_partial_packet(mut s: &str) -> Option<(Packet, &str)> {
    if s.is_empty() || s.starts_with(']') {
        return None;
    }
    if s.starts_with('[') {
        let mut v = vec![];
        let mut s = &s[1..];
        while let Some((packet, rest)) = parse_partial_packet(s) {
            v.push(packet);
            s = rest;
            if s.starts_with(',') {
                s = &s[1..];
            } else {
                break;
            }
        }
        return Some((List(v), &s[1..]));
    }
    let mut n = 0;
    while let Ok(d) = s[0..1].parse::<usize>() {
        n = n * 10 + d;
        s = &s[1..];
    }
    Some((Integer(n), s))
}

fn parse_packet(s: &str) -> Packet {
    let (packet, _) = parse_partial_packet(s).unwrap();
    packet
}

fn parse(input: &str) -> impl Iterator<Item = (Packet, Packet)> + '_ {
    let lines = input.lines().collect::<Vec<_>>();
    let n = lines.len() + 1;
    assert_eq!(n % 3, 0);

    (0..n / 3).map(move |i| {
        let j = i * 3;
        (parse_packet(lines[j]), parse_packet(lines[j + 1]))
    })
}

fn compare(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Integer(x), Integer(y)) => {
            if x == y {
                None
            } else {
                Some(x < y)
            }
        }
        (List(x), List(y)) => {
            let m = x.len();
            let n = y.len();
            for (a, b) in x.into_iter().zip(y.into_iter()) {
                match compare(a, b) {
                    None => continue,
                    Some(b) => return Some(b),
                }
            }
            compare(&Integer(m), &Integer(n))
        }
        (Integer(x), List(_)) => compare(&List(vec![Integer(*x)]), right),
        (List(_), Integer(y)) => compare(left, &List(vec![Integer(*y)])),
    }
}

pub fn puzzle1(input: &str) -> String {
    parse(input)
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if let Some(true) = compare(&left, &right) {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string()
}

pub fn puzzle2(input: &str) -> String {
    let mut packets = parse(input)
        .flat_map(|(left, right)| [left, right])
        .collect::<Vec<_>>();
    let fst = List(vec![List(vec![Integer(2)])]);
    let snd = List(vec![List(vec![Integer(6)])]);
    packets.push(fst.clone());
    packets.push(snd.clone());
    packets.sort_by(|left, right| match compare(left, right) {
        Some(true) => Ordering::Less,
        None => Ordering::Equal,
        Some(false) => Ordering::Greater,
    });
    ((packets.iter().position(|x| *x == fst).unwrap() + 1)
        * (packets.iter().position(|x| *x == snd).unwrap() + 1))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "13");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "5806");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "140");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "23600");
    }
}
