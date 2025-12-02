use ahash::HashSet;
use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let ranges = INPUT
        .trim()
        .split(",")
        .map(|range| match range.split_once("-") {
            Some((start, end)) => Ok((start.parse::<u64>()?, end.parse::<u64>()?)),
            None => anyhow::bail!("invalid range: {range}"),
        })
        .collect::<Result<Vec<_>>>()?;

    println!("part1: {}", compute(&ranges, std::iter::once(2)));
    println!("part2: {}", compute(&ranges, 2..));
    Ok(())
}

fn compute<'a>(
    input: impl IntoIterator<Item = &'a (u64, u64)>,
    n_range: impl IntoIterator<Item = u32> + Clone,
) -> u64 {
    let mut result = 0;

    let mut ranges = [(0, 0, 0), (0, 0, 0)];
    let mut unique = HashSet::default();
    for &(start, end) in input {
        let start_digits = count_digits(start);
        let end_digits = count_digits(end);

        let ranges = if start_digits < end_digits {
            let next_start = 10u64.pow(end_digits - 1);
            ranges[0] = (start_digits, start, next_start - 1);
            ranges[1] = (end_digits, next_start, end);
            &ranges[..]
        } else {
            ranges[0] = (start_digits, start, end);
            &ranges[..1]
        };

        for &(digits, start, end) in ranges {
            unique.clear();

            for n in n_range.clone() {
                if n > digits {
                    break;
                } else if !digits.is_multiple_of(n) {
                    continue;
                }
                let range = start..=end;

                let sub_range = 10u64.pow(digits / n);

                for part in 1..sub_range {
                    let num = (0..n)
                        .into_iter()
                        .map(|n| part * sub_range.pow(n))
                        .sum::<u64>();

                    if range.contains(&num) && unique.insert(num) {
                        result += num;
                    }
                }
            }
        }
    }

    result
}

fn count_digits(value: u64) -> u32 {
    u64::ilog10(value) + 1
}
