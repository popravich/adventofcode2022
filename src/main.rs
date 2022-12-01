use std::env;

use advent2022::{
    day1,
    day2,
    day3,
    day4,
};

static DAY1: &str = include_str!("./input/day01.txt");
static DAY2: &str = include_str!("./input/day01.txt");
static DAY3: &str = include_str!("./input/day01.txt");
static DAY4: &str = include_str!("./input/day01.txt");

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
            let (answer1, answer2) = day2::main(DAY2);
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        3 => {
            let (answer1, answer2) = day3::main(DAY3);
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        4 => {
            let (answer1, answer2) = day4::main(DAY4);
            println!("#1: {}", answer1);
            println!("#2: {}", answer2);
        }
        _ => unimplemented!(),
    }
    Ok(())
}