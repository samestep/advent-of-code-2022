use std::collections::HashSet;

use regex::Regex;

fn parse(input: &str) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + '_ {
    let re =
        Regex::new(r"^Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();
    input.lines().map(move |line| {
        let caps = re.captures(line).unwrap();
        (
            (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        )
    })
}

pub fn puzzle1(input: &str, row: isize) -> String {
    let mut beacons = HashSet::new();
    let mut coverage = HashSet::new();
    for (sensor, beacon) in parse(input) {
        beacons.insert(beacon);
        let d = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
        let h = (row - sensor.1).abs();
        let w = d - h;
        for x in sensor.0 - w..=sensor.0 + w {
            coverage.insert(x);
        }
    }
    for (x, y) in beacons {
        if y == row {
            coverage.remove(&x);
        }
    }
    coverage.len().to_string()
}

fn rect_to_diag(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y)
}

fn diag_to_rect(u: isize, v: isize) -> Option<(isize, isize)> {
    let s = u + v;
    let d = u - v;
    if s % 2 == 0 && d % 2 == 0 {
        Some((s / 2, d / 2))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
    u0: isize,
    u1: isize,
    v0: isize,
    v1: isize,
}

fn subtract(a: Rect, b: Rect) -> Vec<Rect> {
    if b.u1 < a.u0 || a.u1 < b.u0 || b.v1 < a.v0 || a.v1 < b.v0 {
        return vec![a];
    }
    let c = Rect {
        u0: b.u0.max(a.u0),
        u1: b.u1.min(a.u1),
        v0: b.v0.max(a.v0),
        v1: b.v1.min(a.v1),
    };
    let mut rects = vec![];
    for (u0, u1) in [(a.u0, c.u0 - 1), (c.u0, c.u1), (c.u1 + 1, a.u1)] {
        if u0 <= u1 {
            for (v0, v1) in [(a.v0, c.v0 - 1), (c.v0, c.v1), (c.v1 + 1, a.v1)] {
                if v0 <= v1 {
                    let rect = Rect { u0, u1, v0, v1 };
                    if rect != c {
                        rects.push(rect);
                    }
                }
            }
        }
    }
    rects
}

pub fn puzzle2(input: &str, most: isize) -> String {
    let mut rects = vec![Rect {
        u0: 0,
        u1: most * 2,
        v0: -most,
        v1: most,
    }];
    for (sensor, beacon) in parse(input) {
        let (x, y) = sensor;
        let d = (beacon.0 - x).abs() + (beacon.1 - y).abs();
        let (u, v) = rect_to_diag(x, y);
        let b = Rect {
            u0: u - d,
            u1: u + d,
            v0: v - d,
            v1: v + d,
        };
        rects = rects.into_iter().flat_map(|a| subtract(a, b)).collect();
    }
    let points: HashSet<_> = rects
        .into_iter()
        .flat_map(|Rect { u0, u1, v0, v1 }| [(u0, v0), (u0, v1), (u1, v0), (u1, v1)])
        .filter_map(|(u, v)| diag_to_rect(u, v))
        .filter(|&(x, y)| 0 <= x && x <= most && 0 <= y && y <= most)
        .collect();
    assert_eq!(points.len(), 1);
    let (x, y) = points.into_iter().next().unwrap();
    (x * 4000000 + y).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE, 10), "26");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT, 2000000), "4424278");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE, 20), "56000011");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT, 4000000), "10382630753392");
    }
}
