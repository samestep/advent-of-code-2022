pub fn puzzle1(input: &str) -> String {
    let mut n: isize = input
        .lines()
        .map(|l| {
            l.chars().fold(0, |x, c| {
                x * 5
                    + match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!(),
                    }
            })
        })
        .sum();
    let mut v = vec![];
    while n > 0 {
        let d = n % 5;
        v.push(match d {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        });
        n /= 5;
        if d > 2 {
            n += 1;
        }
    }
    v.reverse();
    v.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "2=-1=0");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "2-00=12=21-0=01--000");
    }
}
