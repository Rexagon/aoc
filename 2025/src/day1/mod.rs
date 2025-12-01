use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut result_part1 = 0;
    let mut result_part2 = 0;

    let mut position = 50;
    for line in INPUT.lines() {
        let (prefix, n) = line.split_at(1);

        let mut n = n.parse::<i32>()?;
        match prefix {
            "L" => n = -n,
            "R" => {}
            _ => anyhow::bail!("invalid prefix"),
        };

        result_part2 += n.abs() as usize / 100;
        n %= 100;

        let was_at_zero = position == 0;
        let raw_position = position + 100 + n;
        position = raw_position % 100;

        result_part1 += (position == 0) as usize;
        result_part2 += (!was_at_zero && !(101..=199).contains(&raw_position)) as usize;
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}
