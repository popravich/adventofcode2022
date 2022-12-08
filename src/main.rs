use std::env;

use advent2022::{day1, day2, day3, day4, day5, day6, day7, day8};

static DAY1: &str = include_str!("./input/day01.txt");
static DAY2: &str = include_str!("./input/day02.txt");
static DAY3: &str = include_str!("./input/day03.txt");
static DAY4: &str = include_str!("./input/day04.txt");
static DAY5: &str = include_str!("./input/day05.txt");
static DAY6: &str = include_str!("./input/day06.txt");
static DAY7: &str = include_str!("./input/day07.txt");
static DAY8: &str = include_str!("./input/day08.txt");

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
        _ => unimplemented!(),
    }
    Ok(())
}
