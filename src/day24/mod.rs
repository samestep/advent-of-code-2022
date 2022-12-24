use std::collections::BinaryHeap;

#[derive(PartialEq)]
enum Blizzard {
    U,
    D,
    L,
    R,
}

type Valley = Vec<Vec<Option<Blizzard>>>;

use Blizzard::*;

fn parse(input: &str) -> Valley {
    let lines: Vec<_> = input.lines().collect();
    lines[1..lines.len() - 1]
        .into_iter()
        .map(|l| {
            l.chars()
                .filter_map(|c| match c {
                    '.' => Some(None),
                    '^' => Some(Some(U)),
                    'v' => Some(Some(D)),
                    '<' => Some(Some(L)),
                    '>' => Some(Some(R)),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn get_dims(valley: &Valley) -> (usize, usize) {
    (valley.len(), valley[0].len())
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_clear(valley: &Valley, t: usize, y: usize, x: usize) -> bool {
    let (h, w) = get_dims(valley);
    !(valley[(y + t) % h][x] == Some(U)
        || valley[(y + h - t % h) % h][x] == Some(D)
        || valley[y][(x + t) % w] == Some(L)
        || valley[y][(x + w - t % w) % w] == Some(R))
}

fn search(valley: &Valley, time: usize, start: (usize, usize), end: (usize, usize)) -> usize {
    let (h, w) = get_dims(valley);
    let d = (w * h) / gcd(w, h);
    let mut times = vec![vec![vec![i32::MIN; w]; h]; d];
    let mut queue = BinaryHeap::new();
    for t in time + 1..=time + d {
        if is_clear(&valley, t, start.0, start.1) {
            queue.push((-(t as i32), start));
        }
    }
    while let Some((t, (y, x))) = queue.pop() {
        let i = -t as usize % d;
        if t <= times[i][y][x] {
            continue;
        }
        times[i][y][x] = t;
        if (y, x) == end {
            return (1 - t) as usize;
        }
        if is_clear(&valley, i + 1, y, x) {
            queue.push((t - 1, (y, x)));
        }
        if 0 < y && is_clear(&valley, i + 1, y - 1, x) {
            queue.push((t - 1, (y - 1, x)));
        }
        if y + 1 < h && is_clear(&valley, i + 1, y + 1, x) {
            queue.push((t - 1, (y + 1, x)));
        }
        if 0 < x && is_clear(&valley, i + 1, y, x - 1) {
            queue.push((t - 1, (y, x - 1)));
        }
        if x + 1 < w && is_clear(&valley, i + 1, y, x + 1) {
            queue.push((t - 1, (y, x + 1)));
        }
    }
    panic!()
}

fn solve(input: &str, back: bool) -> usize {
    let valley = parse(input);
    let (h, w) = get_dims(&valley);
    let start = (0, 0);
    let end = (h - 1, w - 1);
    let mut t = search(&valley, 0, start, end);
    if back {
        t = search(&valley, t, end, start);
        t = search(&valley, t, start, end);
    }
    t
}

pub fn puzzle1(input: &str) -> usize {
    solve(input, false)
}

pub fn puzzle2(input: &str) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 18);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 343);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 54);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 960);
    }
}
