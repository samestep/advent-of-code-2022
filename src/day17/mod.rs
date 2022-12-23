use std::collections::{hash_map::Entry, HashMap};

enum Jet {
    L,
    R,
}

use Jet::*;

fn parse(input: &str) -> Vec<Jet> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(L),
            '>' => Some(R),
            _ => None,
        })
        .collect()
}

const ROCKS: [u32; 5] = [
    0b0000000_0_0000000_0_0000000_0_0011110,
    0b0000000_0_0001000_0_0011100_0_0001000,
    0b0000000_0_0000100_0_0000100_0_0011100,
    0b0010000_0_0010000_0_0010000_0_0010000,
    0b0000000_0_0000000_0_0011000_0_0011000,
];

const MASK: u32 = 0b1111111;

struct Falling {
    jets: Vec<Jet>,
    chamber: Vec<u8>,
    j: usize,
}

impl Falling {
    fn get(&self, y: usize) -> u32 {
        let mut rocks = 0;
        for k in (0..4).rev() {
            rocks <<= 8;
            rocks |= *self.chamber.get(y + k).unwrap_or(&0) as u32;
        }
        rocks
    }

    fn set(&mut self, y: usize, rock: u32) {
        while self.chamber.len() < y {
            self.chamber.push(0);
        }
        let mut top = rock;
        for k in 0..4 {
            let row = (top & MASK) as u8;
            if self.chamber.len() <= y + k {
                if row != 0 {
                    self.chamber.push(row);
                }
            } else {
                self.chamber[y + k] |= row;
            }
            top >>= 8;
        }
    }

    fn fall(&mut self, mut rock: u32) -> (u32, usize) {
        let mut y = self.chamber.len() + 3;
        loop {
            let rocks = self.get(y);
            match self.jets[self.j] {
                L => {
                    if rock & 0b1000000_0_1000000_0_1000000_0_1000000 == 0 {
                        let pushed = rock << 1;
                        if pushed & rocks == 0 {
                            rock = pushed;
                        }
                    }
                }
                R => {
                    if rock & 0b0000001_0_0000001_0_0000001_0_0000001 == 0 {
                        let pushed = rock >> 1;
                        if pushed & rocks == 0 {
                            rock = pushed;
                        }
                    }
                }
            }
            self.j = (self.j + 1) % self.jets.len();
            if y == 0 || rock & self.get(y - 1) != 0 {
                break;
            }
            y -= 1;
        }
        (rock, y)
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut falling = Falling {
        jets: parse(input),
        chamber: vec![],
        j: 0,
    };
    for i in 0..2022 {
        let (rock, y) = falling.fall(ROCKS[i % ROCKS.len()]);
        falling.set(y, rock);
    }
    falling.chamber.len()
}

pub fn puzzle2(input: &str) -> usize {
    let mut falling = Falling {
        jets: parse(input),
        chamber: vec![],
        j: 0,
    };
    let mut depth = 0;
    let mut contexts = HashMap::new();
    let mut i = 0;
    loop {
        let key = (i % ROCKS.len(), falling.j);
        let n = falling.chamber.len();
        let val = (i, n, falling.chamber[n - depth..n].to_vec());
        match contexts.entry(key) {
            Entry::Occupied(mut entry) => {
                let before: &(usize, usize, Vec<u8>) = entry.get();
                if before.2 == val.2 {
                    break;
                } else {
                    entry.insert(val);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(val);
            }
        }
        let (rock, y) = falling.fall(ROCKS[i % ROCKS.len()]);
        depth = depth.max(n.saturating_sub(y));
        falling.set(y, rock);
        i += 1;
    }
    let n = falling.chamber.len();
    let (prev_i, prev_n, _) = contexts.get(&(i % ROCKS.len(), falling.j)).unwrap();
    let big_n: usize = 1000000000000 - i;
    let big = (big_n / (i - prev_i)) * (n - prev_n);
    let end = big_n % (i - prev_i);
    for k in 0..end {
        let (rock, y) = falling.fall(ROCKS[(i + k) % ROCKS.len()]);
        falling.set(y, rock);
    }
    big + falling.chamber.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3068);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 3124);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 1514285714288);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1561176470569);
    }
}
