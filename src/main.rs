use std::env;

use advent2022::{day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13};
use advent2022::{day14, day15};

static DAY1: &str = include_str!("./input/day01.txt");
static DAY2: &str = include_str!("./input/day02.txt");
static DAY3: &str = include_str!("./input/day03.txt");
static DAY4: &str = include_str!("./input/day04.txt");
static DAY5: &str = include_str!("./input/day05.txt");
static DAY6: &str = include_str!("./input/day06.txt");
static DAY7: &str = include_str!("./input/day07.txt");
static DAY8: &str = include_str!("./input/day08.txt");
static DAY9: &str = include_str!("./input/day09.txt");
static DAY10: &str = include_str!("./input/day10.txt");
static DAY11: &str = include_str!("./input/day11.txt");
static DAY12: &str = include_str!("./input/day12.txt");
static DAY13: &str = include_str!("./input/day13.txt");
static DAY14: &str = include_str!("./input/day14.txt");
static DAY15: &str = include_str!("./input/day15.txt");

fn main() -> anyhow::Result<()> {
    let day = env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("Day number is required"))
        .and_then(|s| s.trim().parse().map_err(|e| anyhow::anyhow!("{}", e)))?;
    match day {
        1 => {
            let (answer1, answer2) = day1::main(DAY1)?;
            println!("#1: Max calories: {}", answer1);
            println!("#2: Top 3 sum: {}", answer2);
        }
        2 => {
            let (answer1, answer2) = day2::main(DAY2)?;
            println!("#1: Guessed score: {}", answer1);
            println!("#2: Real score: {}", answer2);
        }
        3 => {
            let (answer1, answer2) = day3::main(DAY3)?;
            println!("#1: Priorities sum of duplicates: {}", answer1);
            println!("#2: Priorities sum of badges: {}", answer2);
        }
        4 => {
            let (answer1, answer2) = day4::main(DAY4)?;
            println!("#1: fully contains: {}", answer1);
            println!("#2: overlaps: {}", answer2);
        }
        5 => {
            let (answer1, answer2) = day5::main(DAY5)?;
            println!("#1: fully contains: {}", answer1);
            println!("#2: overlaps: {}", answer2);
        }
        6 => {
            let (answer1, answer2) = day6::main(DAY6)?;
            println!("#1: index: {}", answer1);
            println!("#2: {}", answer2);
        }
        7 => {
            let (answer1, answer2) = day7::main(DAY7)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        8 => {
            let (answer1, answer2) = day8::main(DAY8)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        9 => {
            let (answer1, answer2) = day9::main(DAY9)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        10 => {
            let (answer1, answer2) = day10::main(DAY10)?;
            println!("#1: {}", answer1);
            println!("#2:\n{}", answer2);
        }
        11 => {
            let (answer1, answer2) = day11::main(DAY11)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        12 => {
            let (answer1, answer2) = day12::main(DAY12)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        13 => {
            let (answer1, answer2) = day13::main(DAY13)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        14 => {
            let (answer1, answer2) = day14::main(DAY14)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        15 => {
            let (answer1, answer2) = day15::main(DAY15)?;
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        _ => unimplemented!(),
    }
    Ok(())
}
