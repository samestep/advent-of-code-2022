mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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

        (2, 1) => day02::puzzle1(&input),
        (2, 2) => day02::puzzle2(&input),

        (3, 1) => day03::puzzle1(&input),
        (3, 2) => day03::puzzle2(&input),

        (4, 1) => day04::puzzle1(&input),
        (4, 2) => day04::puzzle2(&input),

        (5, 1) => day05::puzzle1(&input),
        (5, 2) => day05::puzzle2(&input),

        _ => panic!("no puzzle {} for day {}", args.puzzle, args.day),
    };

    println!("{}", answer);
}
