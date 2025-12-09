use anyhow::Result;
use argh::FromArgs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<()> {
    let App { day } = argh::from_env();
    match day {
        0 => anyhow::bail!("day number starts from 1"),
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        8 => day8::run(),
        9 => day9::run(),
        _ => anyhow::bail!("unknown day"),
    }
}

/// AOC 2025
#[derive(FromArgs)]
struct App {
    #[argh(positional)]
    day: usize,
}
