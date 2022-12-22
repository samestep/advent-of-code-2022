mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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

        (6, 1) => day06::puzzle1(&input),
        (6, 2) => day06::puzzle2(&input),

        (7, 1) => day07::puzzle1(&input),
        (7, 2) => day07::puzzle2(&input),

        (8, 1) => day08::puzzle1(&input),
        (8, 2) => day08::puzzle2(&input),

        (9, 1) => day09::puzzle1(&input),
        (9, 2) => day09::puzzle2(&input),

        (10, 1) => day10::puzzle1(&input),
        (10, 2) => day10::puzzle2(&input),

        (11, 1) => day11::puzzle1(&input),
        (11, 2) => day11::puzzle2(&input),

        (12, 1) => day12::puzzle1(&input),
        (12, 2) => day12::puzzle2(&input),

        (13, 1) => day13::puzzle1(&input),
        (13, 2) => day13::puzzle2(&input),

        (14, 1) => day14::puzzle1(&input),
        (14, 2) => day14::puzzle2(&input),

        _ => panic!("no puzzle {} for day {}", args.puzzle, args.day),
    };

    println!("{}", answer);
}
