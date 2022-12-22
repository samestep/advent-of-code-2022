enum Instr {
    Addx(i32),
    Noop,
}

use Instr::*;

fn parse(input: &str) -> impl Iterator<Item = Instr> + '_ {
    input.lines().map(|line| {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "addx" => Addx(words.next().unwrap().parse().unwrap()),
            "noop" => Noop,
            _ => panic!(),
        }
    })
}

trait State {
    fn addx(&mut self, v: i32);
    fn step(&mut self);
}

fn run(instrs: impl Iterator<Item = Instr>, state: &mut impl State) {
    for instr in instrs {
        match instr {
            Addx(v) => {
                state.step();
                state.step();
                state.addx(v);
            }
            Noop => {
                state.step();
            }
        }
    }
}

struct State1 {
    cycle: i32,
    x: i32,
    sum: i32,
}

impl State for State1 {
    fn addx(&mut self, v: i32) {
        self.x += v;
    }

    fn step(&mut self) {
        if self.cycle % 40 == 20 {
            self.sum += self.cycle * self.x;
        }
        self.cycle += 1;
    }
}

pub fn puzzle1(input: &str) -> i32 {
    let mut state = State1 {
        cycle: 1,
        x: 1,
        sum: 0,
    };
    run(parse(input), &mut state);
    state.sum
}

const W: usize = 40;
const H: usize = 6;

struct State2 {
    cycle: i32,
    x: i32,
    crt: [[bool; W]; H],
}

impl State for State2 {
    fn addx(&mut self, v: i32) {
        self.x += v;
    }

    fn step(&mut self) {
        let i = self.cycle as usize - 1;
        let x = i % W;
        let y = i / W;
        if (x as i32 - self.x).abs() <= 1 {
            self.crt[y][x] = true;
        }
        self.cycle += 1;
    }
}

pub fn puzzle2(input: &str) -> String {
    let mut state = State2 {
        cycle: 1,
        x: 1,
        crt: [[false; 40]; 6],
    };
    run(parse(input), &mut state);
    state
        .crt
        .into_iter()
        .flat_map(|row| {
            row.into_iter()
                .map(|b| if b { '#' } else { '.' })
                .chain(['\n'])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
    const IMAGE: &str = include_str!("image.txt");
    const LETTERS: &str = include_str!("letters.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 13140);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 15220);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), IMAGE);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), LETTERS);
    }
}
