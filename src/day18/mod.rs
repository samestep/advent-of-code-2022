use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &str) -> HashSet<(isize, isize, isize)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> usize {
    let cubes = parse(input);
    let mut n = 0;
    for &(x, y, z) in &cubes {
        if !cubes.contains(&(x - 1, y, z)) {
            n += 1;
        }
        if !cubes.contains(&(x + 1, y, z)) {
            n += 1;
        }
        if !cubes.contains(&(x, y - 1, z)) {
            n += 1;
        }
        if !cubes.contains(&(x, y + 1, z)) {
            n += 1;
        }
        if !cubes.contains(&(x, y, z - 1)) {
            n += 1;
        }
        if !cubes.contains(&(x, y, z + 1)) {
            n += 1;
        }
    }
    n
}

pub fn puzzle2(input: &str) -> usize {
    let cubes = parse(input);
    let min = cubes
        .iter()
        .copied()
        .reduce(|(x0, y0, z0), (x1, y1, z1)| (x0.min(x1), y0.min(y1), z0.min(z1)))
        .unwrap();
    let max = cubes
        .iter()
        .copied()
        .reduce(|(x0, y0, z0), (x1, y1, z1)| (x0.max(x1), y0.max(y1), z0.max(z1)))
        .unwrap();
    let size = (
        (5 + max.0 - min.0) as usize,
        (5 + max.1 - min.1) as usize,
        (5 + max.2 - min.2) as usize,
    );
    let mut solid = vec![vec![vec![false; size.2]; size.1]; size.0];
    for (x, y, z) in cubes {
        solid[(x - (min.0 - 2)) as usize][(y - (min.1 - 2)) as usize][(z - (min.2 - 2)) as usize] =
            true;
    }
    let mut outside = vec![vec![vec![false; size.2]; size.1]; size.0];
    for y in 0..size.1 {
        for z in 0..size.2 {
            outside[0][y][z] = true;
            outside[size.0 - 1][y][z] = true;
        }
    }
    for x in 0..size.0 {
        for z in 0..size.2 {
            outside[x][0][z] = true;
            outside[x][size.1 - 1][z] = true;
        }
    }
    for x in 0..size.0 {
        for y in 0..size.1 {
            outside[x][y][0] = true;
            outside[x][y][size.2 - 1] = true;
        }
    }
    let mut stack = vec![(1, 1, 1)];
    while let Some((x, y, z)) = stack.pop() {
        if solid[x][y][z] || outside[x][y][z] {
            continue;
        }
        outside[x][y][z] = true;
        stack.push((x - 1, y, z));
        stack.push((x + 1, y, z));
        stack.push((x, y - 1, z));
        stack.push((x, y + 1, z));
        stack.push((x, y, z - 1));
        stack.push((x, y, z + 1));
    }
    let mut n = 0;
    for x in 2..size.0 - 2 {
        for y in 2..size.1 - 2 {
            for z in 2..size.2 - 2 {
                if solid[x][y][z] {
                    if outside[x - 1][y][z] {
                        n += 1;
                    }
                    if outside[x + 1][y][z] {
                        n += 1;
                    }
                    if outside[x][y - 1][z] {
                        n += 1;
                    }
                    if outside[x][y + 1][z] {
                        n += 1;
                    }
                    if outside[x][y][z - 1] {
                        n += 1;
                    }
                    if outside[x][y][z + 1] {
                        n += 1;
                    }
                }
            }
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), 10);
    }

    #[test]
    fn test_puzzle1_example2() {
        assert_eq!(puzzle1(EXAMPLE2), 64);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 3542);
    }

    #[test]
    fn test_puzzle2_example1() {
        assert_eq!(puzzle2(EXAMPLE1), 10);
    }

    #[test]
    fn test_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), 58);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 2080);
    }
}
