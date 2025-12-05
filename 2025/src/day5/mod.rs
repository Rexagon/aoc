use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut lines = INPUT.trim().lines();
    let mut ranges = Vec::new();
    for item in lines.by_ref() {
        if item.trim().is_empty() {
            break;
        }

        let (start, end) = match item.split_once("-") {
            Some((s, e)) => (s.parse::<u64>()?, e.parse::<u64>()?),
            None => anyhow::bail!("invalid range"),
        };
        assert!(start <= end);
        ranges.push((start, end));
    }

    let mut max_id = 0u64;
    let mut result_part2 = 0u64;
    ranges.sort_unstable();
    ranges.retain_mut(|(start, end)| {
        *start = std::cmp::max(*start, max_id);
        if *start > *end || *end < max_id {
            return false;
        }
        max_id = *end + 1;
        result_part2 += max_id - *start;
        true
    });

    let mut ids = Vec::new();
    for item in lines {
        ids.push(item.trim().parse::<u64>()?);
    }

    let mut result_part1 = 0usize;
    'outer: for id in ids {
        for (start, end) in &ranges {
            if (*start..=*end).contains(&id) {
                result_part1 += 1;
                continue 'outer;
            }
        }
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}
