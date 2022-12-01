mod day01;

use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Args {
    day: u8,
    puzzle: u8,
    input: String,
}

fn main() {
    let args = Args::parse();
    let input = fs::read_to_string(args.input).unwrap();

    let answer = match (args.day, args.puzzle) {
        (1, 1) => day01::puzzle1(&input),
        (1, 2) => day01::puzzle2(&input),

        _ => panic!("no puzzle {} for day {}", args.puzzle, args.day),
    };

    println!("{}", answer);
}
