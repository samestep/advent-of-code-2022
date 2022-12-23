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

pub fn puzzle1(input: &str) -> usize {
    let jets = parse(input);
    let mut chamber = Vec::<u8>::new();
    let mut j = 0;
    for i in 0..2022 {
        let mut rock = ROCKS[i % 5];
        let mut y = chamber.len() + 3;
        'fall: loop {
            let mut rocks = 0;
            for k in (0..4).rev() {
                rocks <<= 8;
                if y + k < chamber.len() {
                    rocks |= chamber[y + k] as u32;
                }
            }
            match jets[j] {
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
            j = (j + 1) % jets.len();
            if y == 0 {
                break 'fall;
            }
            let mut top = rock;
            for k in (y - 1)..chamber.len().min(y + 3) {
                let row = (top & MASK) as u8;
                if k < chamber.len() && row & chamber[k] != 0 {
                    break 'fall;
                }
                top >>= 8;
            }
            y -= 1;
        }
        while chamber.len() < y {
            chamber.push(0);
        }
        let mut top = rock;
        for k in 0..4 {
            let row = (top & MASK) as u8;
            if chamber.len() <= y + k {
                if row != 0 {
                    chamber.push(row);
                }
            } else {
                chamber[y + k] |= row;
            }
            top >>= 8;
        }
    }
    chamber.len()
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
}
