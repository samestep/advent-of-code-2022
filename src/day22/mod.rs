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

enum Facing {
    R,
    D,
    L,
    U,
}

use Facing::*;

pub fn puzzle1(input: &str) -> usize {
    let (grid, movements) = parse(input);
    let rows: Vec<_> = grid
        .iter()
        .map(|row| {
            let it = row.iter().enumerate().filter_map(|(j, t)| t.map(|_| j));
            (it.clone().min(), it.max())
        })
        .collect();
    let cols: Vec<_> = (0..grid[0].len())
        .map(|j| {
            let it = grid
                .iter()
                .enumerate()
                .filter_map(|(i, row)| row[j].map(|_| i));
            (it.clone().min(), it.max())
        })
        .collect();
    let mut y = 0;
    let mut x = rows[y].0.unwrap();
    let mut facing = R;
    for movement in movements {
        match movement {
            Go(n) => {
                for _ in 0..n {
                    let (mut i, mut j) = match facing {
                        R => (y, x + 1),
                        D => (y + 1, x),
                        L => (y, x - 1),
                        U => (y - 1, x),
                    };
                    if grid
                        .get(i)
                        .and_then(|row| row.get(j).copied())
                        .flatten()
                        .is_none()
                    {
                        match facing {
                            R => j = rows[y].0.unwrap(),
                            D => i = cols[x].0.unwrap(),
                            L => j = rows[y].1.unwrap(),
                            U => i = cols[x].1.unwrap(),
                        }
                    }
                    if let Some(Some(false)) = grid.get(i).and_then(|row| row.get(j)) {
                        (x, y) = (j, i);
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
}
