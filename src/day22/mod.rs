use std::collections::{HashMap, HashSet};

use regex::Regex;

enum Wise {
    L,
    R,
}

enum Movement {
    Go(usize),
    Turn(Wise),
}

use Movement::*;

fn parse(input: &str) -> (Vec<Vec<Option<bool>>>, Vec<Movement>) {
    let lines: Vec<_> = input.lines().collect();
    let i = lines.len() - 2;
    let w = lines[..i].iter().map(|line| line.len()).max().unwrap();
    (
        lines[..i]
            .iter()
            .map(|line| {
                let mut row = vec![None; w];
                for (j, c) in line.chars().enumerate() {
                    if c != ' ' {
                        row[j] = Some(c == '#');
                    }
                }
                row
            })
            .collect(),
        Regex::new(r"\d+|[LR]")
            .unwrap()
            .captures_iter(lines[i + 1])
            .map(|cap| match &cap[0] {
                "L" => Turn(Wise::L),
                "R" => Turn(Wise::R),
                n => Go(n.parse().unwrap()),
            })
            .collect(),
    )
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Dir {
    R,
    D,
    L,
    U,
}

use Dir::*;

impl Dir {
    fn right(self) -> Self {
        match self {
            R => D,
            D => L,
            L => U,
            U => R,
        }
    }

    fn flip(self) -> Self {
        match self {
            R => L,
            D => U,
            L => R,
            U => D,
        }
    }

    fn left(self) -> Self {
        match self {
            R => U,
            D => R,
            L => D,
            U => L,
        }
    }
}

fn get(grid: &[Vec<Option<bool>>], x: isize, y: isize) -> Option<bool> {
    let i: usize = y.try_into().ok()?;
    let j: usize = x.try_into().ok()?;
    grid.get(i).and_then(|row| row.get(j).copied()).flatten()
}

fn walk(
    grid: &[Vec<Option<bool>>],
    mut x: isize,
    mut y: isize,
    movements: &[Movement],
    wrap: impl Fn(isize, isize, Dir) -> (isize, isize, Dir),
) -> isize {
    let mut dir = R;
    for movement in movements {
        match movement {
            &Go(n) => {
                for _ in 0..n {
                    let (mut i, mut j) = match dir {
                        R => (y, x + 1),
                        D => (y + 1, x),
                        L => (y, x - 1),
                        U => (y - 1, x),
                    };
                    let mut d = dir;
                    if get(grid, j, i).is_none() {
                        (j, i, d) = wrap(x, y, dir);
                    }
                    if let Some(false) = get(grid, j, i) {
                        (x, y, dir) = (j, i, d);
                    }
                }
            }
            Turn(wise) => {
                dir = match wise {
                    Wise::L => dir.left(),
                    Wise::R => dir.right(),
                }
            }
        }
    }
    1000 * (y + 1)
        + 4 * (x + 1)
        + match dir {
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        }
}

pub fn puzzle1(input: &str) -> isize {
    let (grid, movements) = parse(input);
    let rows: Vec<_> = grid
        .iter()
        .map(|row| {
            let it = row.iter().enumerate().filter_map(|(j, t)| t.map(|_| j));
            (it.clone().min().unwrap(), it.max().unwrap())
        })
        .collect();
    let cols: Vec<_> = (0..grid[0].len())
        .map(|j| {
            let it = grid
                .iter()
                .enumerate()
                .filter_map(|(i, row)| row[j].map(|_| i));
            (it.clone().min().unwrap(), it.max().unwrap())
        })
        .collect();
    let y = 0;
    let x = rows[y as usize].0 as isize;
    walk(&grid, x, y, &movements, |mut j, mut i, dir| {
        match dir {
            R => j = rows[i as usize].0 as isize,
            D => i = cols[j as usize].0 as isize,
            L => j = rows[i as usize].1 as isize,
            U => i = cols[j as usize].1 as isize,
        }
        (j, i, dir)
    })
}

type Edge = ((isize, isize), Dir);
type Face = HashMap<Dir, Edge>;
type Cube = HashMap<(isize, isize), Face>;

fn join(f1: &mut Face, e1: Edge, f2: &mut Face, e2: Edge) {
    f1.insert(e1.1, e2);
    f2.insert(e2.1, e1);
}

fn fold(map: HashSet<(isize, isize)>) -> Cube {
    let mut faces: Cube = HashMap::new();
    let mut stack = vec![*map.iter().next().unwrap()];
    while let Some(k0) = stack.pop() {
        let (x0, y0) = k0;
        let mut f0 = HashMap::new();
        for (d0, k1) in [
            (R, (x0 + 1, y0)),
            (D, (x0, y0 + 1)),
            (L, (x0 - 1, y0)),
            (U, (x0, y0 - 1)),
        ] {
            if !map.contains(&k1) {
                continue;
            }
            if let Some(f1) = faces.get_mut(&k1) {
                join(&mut f0, (k0, d0), f1, (k1, d0.flip()));
                let (l, r) = (f1.get(&d0.left()).copied(), f1.get(&d0.right()).copied());
                let mut follow = |e, r0: fn(Dir) -> Dir, r: fn(Dir) -> Dir| {
                    if let Some((k2, d2)) = e {
                        let f2 = faces.get_mut(&k2).unwrap();
                        join(&mut f0, (k0, r0(d0)), f2, (k2, r(d2)));
                        if let Some(&(k3, d3)) = f2.get(&d2.flip()) {
                            let f3 = faces.get_mut(&k3).unwrap();
                            join(&mut f0, (k0, d0.flip()), f3, (k3, r(d3)));
                        }
                    }
                };
                follow(l, |d| d.left(), |d| d.right());
                follow(r, |d| d.right(), |d| d.left());
            } else {
                stack.push(k1);
            }
        }
        faces.insert(k0, f0);
    }
    faces
}

fn isqrt(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    let mut x = 1 << ((1 + usize::BITS - n.leading_zeros()) / 2);
    loop {
        let y = (x + n / x) / 2;
        if x <= y {
            return x;
        }
        x = y;
    }
}

pub fn puzzle2(input: &str) -> isize {
    let (grid, movements) = parse(input);
    let n = grid
        .iter()
        .flat_map(|row| row.iter().filter(|tile| tile.is_some()))
        .count();
    let s = isqrt(n / 6);
    let (h, w) = (grid.len(), grid[0].len());
    assert_eq!(6 * s * s, n);
    assert_eq!(h % s, 0);
    assert_eq!(w % s, 0);
    let cube = fold(
        (0..h / s)
            .flat_map(|i| {
                let map = &grid; // https://stackoverflow.com/a/67230904/5044950
                (0..w / s).filter_map(move |j| map[i * s][j * s].map(|_| (j as isize, i as isize)))
            })
            .collect(),
    );
    walk(
        &grid,
        grid[0].iter().position(|tile| tile.is_some()).unwrap() as isize,
        0,
        &movements,
        |x0, y0, mut d0| {
            let l = s as isize;
            let k0 = (x0 / l, y0 / l);
            let ((j, i), d1) = cube[&k0][&d0];
            let (mut x, mut y) = (x0 % l, y0 % l);
            while d0 != d1.flip() {
                (x, y) = (l - 1 - y, x);
                d0 = d0.right();
            }
            let (x1, y1) = match d1 {
                R => (l - 1, y),
                D => (x, l - 1),
                L => (0, y),
                U => (x, 0),
            };
            (j * l + x1, i * l + y1, d1.flip())
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 6032);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 20494);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 5031);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 55343);
    }
}
