fn list_elves(input: &str) -> Vec<i32> {
    let mut elves = vec![];
    let mut elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<i32>().unwrap();
        }
    }
    elves.push(elf);
    elves
}

pub fn puzzle1(input: &str) -> String {
    list_elves(input).into_iter().max().unwrap().to_string()
}

pub fn puzzle2(input: &str) -> String {
    let mut elves = list_elves(input);
    elves.sort_by_key(|x| -x);
    (elves[0] + elves[1] + elves[2]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "24000");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "75622");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "45000");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "213159");
    }
}
