use anyhow::Result;

static INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let (lines, last_line) = INPUT.rsplit_once("\n").unwrap();
    let lines = lines.lines().map(str::as_bytes).collect::<Vec<_>>();

    let mut row_len = 0usize;
    for line in &lines {
        assert!(row_len == 0 || row_len == line.len());
        row_len = line.len();
    }

    let mut last_line = last_line.as_bytes().to_vec();
    last_line.resize(row_len, b' ');

    let mut result_part1 = 0;
    let mut result_part2 = 0;

    let mut start = 0;
    while start < last_line.len() {
        let op = match last_line[start] {
            b'+' => Op::Add,
            b'*' => Op::Mul,
            _ => anyhow::bail!("unknown operation"),
        };

        let mut end = start + 1;
        while end < last_line.len() && last_line[end].is_ascii_whitespace() {
            end += 1;
        }

        result_part1 += process_block(
            op,
            lines.iter().map(|l| l[start..end].iter().rev().copied()),
        );

        result_part2 += process_block(
            op,
            (start..end).map(|i| lines.iter().rev().map(move |l| l[i])),
        );

        start = end;
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}

fn process_block<L, C>(op: Op, lines: L) -> u64
where
    L: Iterator<Item = C>,
    C: Iterator<Item = u8>,
{
    let mut acc = match op {
        Op::Add => 0,
        Op::Mul => 1,
    };

    for line in lines {
        let mut n = 0;
        let mut pow = 1u64;
        let mut has_number = false;
        for c in line {
            if c.is_ascii_whitespace() {
                continue;
            }

            has_number = true;
            n += pow * (c - b'0') as u64;
            pow *= 10;
        }

        if has_number {
            op.apply(&mut acc, n);
        }
    }

    acc
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(self, acc: &mut u64, n: u64) {
        match self {
            Self::Add => *acc += n,
            Self::Mul => *acc *= n,
        }
    }
}
