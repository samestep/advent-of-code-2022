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

pub fn puzzle1(input: &str) -> i32 {
    list_elves(input).into_iter().max().unwrap()
}

pub fn puzzle2(input: &str) -> i32 {
    let mut elves = list_elves(input);
    elves.sort_by_key(|x| -x);
    elves[0] + elves[1] + elves[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/day01.txt");

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(INPUT), 75622);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(INPUT), 213159);
    }
}
