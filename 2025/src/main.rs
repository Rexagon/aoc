use anyhow::Result;
use argh::FromArgs;

mod day1;

fn main() -> Result<()> {
    let App { day } = argh::from_env();
    match day {
        0 => anyhow::bail!("day number start from 1"),
        1 => day1::run(),
        _ => anyhow::bail!("unknown day"),
    }
}

/// AOC 2025
#[derive(FromArgs)]
struct App {
    #[argh(positional)]
    day: usize,
}
