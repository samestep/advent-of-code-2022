fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> u32 {
    let grid = parse(input);
    let mut visible = vec![vec![false; grid[0].len()]; grid.len()];

    // up
    for j in 0..grid[0].len() {
        let mut tallest = -1;
        for i in 0..grid.len() {
            if grid[i][j] > tallest {
                visible[i][j] = true;
            }
            tallest = tallest.max(grid[i][j]);
        }
    }

    // down
    for j in 0..grid[0].len() {
        let mut tallest = -1;
        for i in (0..grid.len()).rev() {
            if grid[i][j] > tallest {
                visible[i][j] = true;
            }
            tallest = tallest.max(grid[i][j]);
        }
    }

    // left
    for i in 0..grid.len() {
        let mut tallest = -1;
        for j in 0..grid[i].len() {
            if grid[i][j] > tallest {
                visible[i][j] = true;
            }
            tallest = tallest.max(grid[i][j]);
        }
    }

    // right
    for i in 0..grid.len() {
        let mut tallest = -1;
        for j in (0..grid[i].len()).rev() {
            if grid[i][j] > tallest {
                visible[i][j] = true;
            }
            tallest = tallest.max(grid[i][j]);
        }
    }

    visible
        .into_iter()
        .map(|v| v.into_iter().map(|x| if x { 1 } else { 0 }).sum::<u32>())
        .sum()
}

pub fn puzzle2(input: &str) -> u32 {
    let grid = parse(input);
    let mut scenic = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut up = 0;
            for k in (0..i).rev() {
                up += 1;
                if grid[i][j] <= grid[k][j] {
                    break;
                }
            }

            let mut down = 0;
            for k in (i + 1)..grid.len() {
                down += 1;
                if grid[i][j] <= grid[k][j] {
                    break;
                }
            }

            let mut left = 0;
            for k in (0..j).rev() {
                left += 1;
                if grid[i][j] <= grid[i][k] {
                    break;
                }
            }

            let mut right = 0;
            for k in (j + 1)..grid[i].len() {
                right += 1;
                if grid[i][j] <= grid[i][k] {
                    break;
                }
            }

            scenic = scenic.max(up * down * left * right);
        }
    }
    scenic
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 21);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 1647);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 8);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 392080);
    }
}
