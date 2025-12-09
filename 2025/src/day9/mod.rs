use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut coords = Vec::new();
    for line in INPUT.trim().lines() {
        let coord = match line.split_once(",") {
            Some((x, y)) => (x.parse::<i64>()?, y.parse::<i64>()?),
            None => anyhow::bail!("invalid coord"),
        };
        coords.push(coord);
    }

    let n = coords.len();
    let mut squares = Vec::with_capacity((n * n - n) / 2);
    for i in 0..n {
        for j in 0..i {
            let s = square(coords[i], coords[j]);
            squares.push((s, i, j));
        }
    }
    squares.sort_unstable_by(|(a, ..), (b, ..)| b.cmp(a));
    let result_part1 = squares[0].0;

    let mut result_part2 = 0;
    for &(s, i, j) in &squares {
        let c1 = coords[i];
        let c2 = coords[j];
        let op1 = (c1.0, c2.1);
        let op2 = (c2.0, c1.1);

        let mut iter = coords.iter().cycle().take(n + 1);
        let mut prev_coord = *iter.next().unwrap();

        let edges = [(c1, op1), (op1, c2), (c2, op2), (op2, c1)];

        let mut inside = true;
        'coords: for &coord in iter {
            for (e1, e2) in edges {
                if intersects(prev_coord, coord, e1, e2) {
                    inside = false;
                    break 'coords;
                }
            }
            prev_coord = coord;
        }

        if inside {
            result_part2 = s;
            break;
        }
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}

fn intersects(a1: Coord, a2: Coord, b1: Coord, b2: Coord) -> bool {
    if (a1.0 == a2.0) == (b1.0 == b2.0) {
        return false;
    }

    let t1 = (a1.0 - b1.0) * (b1.1 - b2.1) - (a1.1 - b1.1) * (b1.0 - b2.0);
    let t2 = (a1.0 - a2.0) * (b1.1 - b2.1) - (a1.1 - a2.1) * (b1.0 - b2.0);
    let u1 = -(a1.0 - a2.0) * (a1.1 - b1.1) + (a1.1 - a2.1) * (a1.0 - b1.0);

    u1 != 0
        && t1 != 0
        && (t1 < 0) == (t2 < 0)
        && t1.abs() <= t2.abs()
        && (u1 < 0) == (t2 < 0)
        && u1.abs() < t2.abs()
}

fn square(a: Coord, b: Coord) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

type Coord = (i64, i64);
