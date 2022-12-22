use std::collections::BinaryHeap;

struct Heightmap {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_char(c: char) -> u8 {
    c as u8 - 'a' as u8
}

fn parse(input: &str) -> Heightmap {
    let mut start = None;
    let mut end = None;
    Heightmap {
        grid: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((y, x));
                            parse_char('a')
                        }
                        'E' => {
                            end = Some((y, x));
                            parse_char('z')
                        }
                        _ => parse_char(c),
                    })
                    .collect()
            })
            .collect(),
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn solve(
    grid: Vec<Vec<u8>>,
    mut queue: BinaryHeap<(isize, (usize, usize))>,
    end: (usize, usize),
) -> String {
    let h = grid.len();
    let w = grid[0].len();
    let mut visited = vec![vec![None; w]; h];
    while let Some((d, (y, x))) = queue.pop() {
        if let Some(_) = visited[y][x] {
            continue;
        }
        visited[y][x] = Some(d);
        if 0 < y && grid[y - 1][x] <= grid[y][x] + 1 {
            queue.push((d - 1, (y - 1, x)));
        }
        if y + 1 < h && grid[y + 1][x] <= grid[y][x] + 1 {
            queue.push((d - 1, (y + 1, x)));
        }
        if 0 < x && grid[y][x - 1] <= grid[y][x] + 1 {
            queue.push((d - 1, (y, x - 1)));
        }
        if x + 1 < w && grid[y][x + 1] <= grid[y][x] + 1 {
            queue.push((d - 1, (y, x + 1)));
        }
    }
    let (y, x) = end;
    (-visited[y][x].unwrap()).to_string()
}

pub fn puzzle1(input: &str) -> String {
    let Heightmap { grid, start, end } = parse(input);
    let mut queue = BinaryHeap::new();
    queue.push((0, start));
    solve(grid, queue, end)
}

pub fn puzzle2(input: &str) -> String {
    let Heightmap { grid, end, .. } = parse(input);
    let mut queue = BinaryHeap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                queue.push((0, (y, x)));
            }
        }
    }
    solve(grid, queue, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "31");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "370");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "29");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "363");
    }
}
