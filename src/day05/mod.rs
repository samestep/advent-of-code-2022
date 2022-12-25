use regex::Regex;

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

struct Drawing {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parse(input: &str) -> Drawing {
    let lines = input.lines().collect::<Vec<_>>();
    let i = lines.iter().position(|&line| line.is_empty()).unwrap();

    // assume there aren't more than 9 stacks
    let mut n = 0;
    while 1 + (n * 4) < lines[i - 1].len() {
        n += 1;
    }

    let mut crates = vec![vec![]; n];
    for j in (0..(i - 1)).rev() {
        let line = lines[j].chars().collect::<Vec<_>>();
        let mut k = 0;
        while k < n {
            if let Some(&c) = line.get(1 + (k * 4)) {
                if c != ' ' {
                    crates[k].push(c);
                }
            }
            k += 1;
        }
    }

    let mut moves = vec![];
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for &line in &lines[(i + 1)..] {
        let cap = re.captures(line).unwrap();
        moves.push(Move {
            quantity: cap[1].parse().unwrap(),
            from: cap[2].parse::<usize>().unwrap() - 1,
            to: cap[3].parse::<usize>().unwrap() - 1,
        });
    }

    Drawing { crates, moves }
}

fn tops(crates: Vec<Vec<char>>) -> String {
    crates
        .into_iter()
        .map(|c| c.last().unwrap().to_string())
        .collect()
}

pub fn puzzle1(input: &str) -> String {
    let Drawing { mut crates, moves } = parse(input);
    for Move { quantity, from, to } in moves {
        let mut i = 0;
        while i < quantity {
            let c = crates[from].pop().unwrap();
            crates[to].push(c);
            i += 1;
        }
    }
    tops(crates)
}

fn two_refs<T>(v: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    if j < i {
        let (l, r) = two_refs(v, j, i);
        return (r, l);
    }
    if i == j {
        panic!("i == j");
    }
    let (l, r) = v.split_at_mut(j);
    (&mut l[i], &mut r[0])
}

pub fn puzzle2(input: &str) -> String {
    let Drawing { mut crates, moves } = parse(input);
    for Move { quantity, from, to } in moves {
        let (f, t) = two_refs(&mut crates, from, to);
        let i = f.len() - quantity;
        t.extend_from_slice(f[i..].as_ref());
        f.truncate(i);
    }
    tops(crates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), "FZCMJCRHZ");
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), "MCD");
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), "JSDHQMZGF");
    }
}
