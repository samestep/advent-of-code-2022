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
    (
        {
            let w = lines[..i].iter().map(|line| line.len()).max().unwrap();
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
                .collect()
        },
        {
            let mut movements = vec![];
            let mut n = 0;
            for c in lines[i + 1].chars() {
                match c {
                    'L' => {
                        if n != 0 {
                            movements.push(Go(n));
                            n = 0;
                        }
                        movements.push(Turn(Wise::L));
                    }
                    'R' => {
                        if n != 0 {
                            movements.push(Go(n));
                            n = 0;
                        }
                        movements.push(Turn(Wise::R));
                    }
                    d => {
                        n = n * 10 + d.to_digit(10).unwrap() as usize;
                    }
                }
            }
            if n != 0 {
                movements.push(Go(n))
            }
            movements
        },
    )
}

#[derive(Clone, Copy)]
enum Facing {
    R,
    D,
    L,
    U,
}

use Facing::*;

fn get(grid: &[Vec<Option<bool>>], x: isize, y: isize) -> Option<bool> {
    grid.get(y as usize)
        .and_then(|row| row.get(x as usize).copied())
        .flatten()
}

fn walk(
    grid: &[Vec<Option<bool>>],
    mut x: isize,
    mut y: isize,
    movements: &[Movement],
    wrap: impl Fn(isize, isize, Facing) -> (isize, isize, Facing),
) -> isize {
    let mut facing = R;
    for movement in movements {
        match movement {
            Go(n) => {
                for _ in 0..*n {
                    let (mut i, mut j) = match facing {
                        R => (y, x + 1),
                        D => (y + 1, x),
                        L => (y, x - 1),
                        U => (y - 1, x),
                    };
                    let mut f = facing;
                    if get(grid, j, i).is_none() {
                        (j, i, f) = wrap(x, y, facing);
                    }
                    if let Some(false) = get(grid, j, i) {
                        (x, y, facing) = (j, i, f);
                    }
                }
            }
            Turn(wise) => {
                facing = match (facing, wise) {
                    (R, Wise::L) => U,
                    (D, Wise::L) => R,
                    (L, Wise::L) => D,
                    (U, Wise::L) => L,
                    (R, Wise::R) => D,
                    (D, Wise::R) => L,
                    (L, Wise::R) => U,
                    (U, Wise::R) => R,
                }
            }
        }
    }
    1000 * (y + 1)
        + 4 * (x + 1)
        + match facing {
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
    walk(&grid, x, y, &movements, |mut j, mut i, facing| {
        match facing {
            R => j = rows[i as usize].0 as isize,
            D => i = cols[j as usize].0 as isize,
            L => j = rows[i as usize].1 as isize,
            U => i = cols[j as usize].1 as isize,
        }
        (j, i, facing)
    })
}

const S: isize = 50;

pub fn puzzle2(input: &str) -> isize {
    let (grid, movements) = parse(input);
    walk(&grid, S, 0, &movements, |x, y, facing| {
        match (x / S, y / S, facing) {
            (2, 0, R) => (2 * S - 1, 3 * S - 1 - y % S, L),
            (1, 2, R) => (3 * S - 1, S - 1 - y % S, L),

            (2, 0, D) => (2 * S - 1, S + x % S, L),
            (1, 1, R) => (2 * S + y % S, S - 1, U),

            (1, 2, D) => (S - 1, 3 * S + x % S, L),
            (0, 3, R) => (S + y % S, 3 * S - 1, U),

            (0, 3, D) => (2 * S + x % S, 0, D),
            (2, 0, U) => (x % S, 4 * S - 1, U),

            (0, 3, L) => (S + y % S, 0, D),
            (1, 0, U) => (0, 3 * S + x % S, R),

            (0, 2, L) => (S, S - 1 - y % S, R),
            (1, 0, L) => (0, 3 * S - 1 - y % S, R),

            (0, 2, U) => (S, S + x % S, R),
            (1, 1, L) => (y % S, 2 * S, D),

            _ => unimplemented!(),
        }
    })
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
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 55343);
    }
}
