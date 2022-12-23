fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve<const M: usize>(numbers: &mut [isize]) -> isize {
    let n = numbers.len();
    let mut forward = (0..n).collect::<Vec<_>>();
    let mut backward = forward.clone();
    for _ in 0..M {
        for i in 0..n {
            let j = forward[i];
            let d = numbers[j];
            let k = ((j as isize + d - 1).rem_euclid((n - 1) as isize) + 1) as usize;
            if j < k {
                for l in j..k {
                    numbers[l] = numbers[l + 1];
                    forward[backward[l + 1]] -= 1;
                    backward[l] = backward[l + 1];
                }
            } else {
                for l in (k..j).rev() {
                    numbers[l + 1] = numbers[l];
                    forward[backward[l]] += 1;
                    backward[l + 1] = backward[l];
                }
            }
            numbers[k] = d;
            forward[i] = k;
            backward[k] = i;
        }
    }
    let i = numbers.iter().position(|&x| x == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|k| numbers[(i + k) % n])
        .sum()
}

pub fn puzzle1(input: &str) -> isize {
    solve::<1>(&mut parse(input))
}

pub fn puzzle2(input: &str) -> isize {
    let mut v = parse(input);
    for i in 0..v.len() {
        v[i] *= 811589153;
    }
    solve::<10>(&mut v)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 7153);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 1623178306);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6146976244822);
    }
}
