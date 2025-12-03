use anyhow::Result;
use argh::FromArgs;

mod day1;
mod day2;
mod day3;

fn main() -> Result<()> {
    let App { day } = argh::from_env();
    match day {
        0 => anyhow::bail!("day number starts from 1"),
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        _ => anyhow::bail!("unknown day"),
    }
}

/// AOC 2025
#[derive(FromArgs)]
struct App {
    #[argh(positional)]
    day: usize,
}
