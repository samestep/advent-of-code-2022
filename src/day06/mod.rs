use std::collections::HashSet;

fn solve(s: &str, n: usize) -> String {
    let v = s.chars().collect::<Vec<_>>();
    for i in 0..=(v.len() - n) {
        if v[i..i + n].iter().copied().collect::<HashSet<_>>().len() == n {
            return (i + n).to_string();
        }
    }
    panic!()
}

pub fn puzzle1(input: &str) -> String {
    solve(input, 4)
}

pub fn puzzle2(input: &str) -> String {
    solve(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE0: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example0() {
        assert_eq!(puzzle1(EXAMPLE0), "7");
    }

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), "5");
    }

    #[test]
    fn test_puzzle1_example2() {
        assert_eq!(puzzle1(EXAMPLE2), "6");
    }

    #[test]
    fn test_puzzle1_example3() {
        assert_eq!(puzzle1(EXAMPLE3), "10");
    }

    #[test]
    fn test_puzzle1_example4() {
        assert_eq!(puzzle1(EXAMPLE4), "11");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "1093");
    }

    #[test]
    fn test_puzzle2_example0() {
        assert_eq!(puzzle2(EXAMPLE0), "19");
    }

    #[test]
    fn test_puzzle2_example1() {
        assert_eq!(puzzle2(EXAMPLE1), "23");
    }

    #[test]
    fn test_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), "23");
    }

    #[test]
    fn test_puzzle2_example3() {
        assert_eq!(puzzle2(EXAMPLE3), "29");
    }

    #[test]
    fn test_puzzle2_example4() {
        assert_eq!(puzzle2(EXAMPLE4), "26");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "3534");
    }
}
