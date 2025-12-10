use std::str::FromStr;

use anyhow::{Context, Result};
use z3::ast::Int;
use z3::{Optimize, SatResult};

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut result_part1 = 0;
    let mut result_part2 = 0;

    let mut matrix = Vec::new();
    for line in INPUT.trim().lines() {
        let mut parts = line.split_whitespace();
        let lights = remove_braces(parts.next().unwrap(), '[', ']')
            .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())?;
        let joltage =
            remove_braces(parts.next_back().unwrap(), '{', '}').and_then(parse_list::<u64>)?;
        assert_eq!(lights.len(), joltage.len());

        let buttons = parts
            .map(|part| remove_braces(part, '(', ')').and_then(parse_list::<usize>))
            .collect::<Result<Vec<_>>>()?;

        let rows = lights.len();
        let columns = buttons.len();

        matrix.clear();
        matrix.resize(rows * columns, false);
        for (col, light_indices) in buttons.iter().enumerate() {
            for row in light_indices {
                matrix[*row * columns + col] = true;
            }
        }

        let xs = (0..buttons.len())
            .map(|i| Int::fresh_const(&format!("x{i}")))
            .collect::<Vec<_>>();

        let solver_part1 = Optimize::new();
        let solver_part2 = Optimize::new();
        for x in &xs {
            solver_part1.assert(&x.ge(0));
            solver_part2.assert(&x.ge(0));
        }

        for row in 0..rows {
            let mut expr = None;
            for col in 0..columns {
                if matrix[row * columns + col] {
                    match &mut expr {
                        None => expr = Some(xs[col].clone()),
                        Some(expr) => *expr += &xs[col],
                    }
                }
            }

            if let Some(expr) = expr {
                solver_part1.assert(&expr.modulo(2).eq(lights[row] as i32));
                solver_part2.assert(&expr.eq(joltage[row]));
            }
        }

        let total_presses = xs
            .iter()
            .fold(None, |prev, x| {
                Some(match prev {
                    None => x.clone(),
                    Some(prev) => prev + x,
                })
            })
            .unwrap();
        solver_part1.minimize(&total_presses);
        solver_part2.minimize(&total_presses);

        anyhow::ensure!(
            solver_part1.check(&[]) == SatResult::Sat,
            "unsatisfied part1"
        );
        anyhow::ensure!(
            solver_part2.check(&[]) == SatResult::Sat,
            "unsatisfied part2"
        );

        let model_part1 = solver_part1.get_model().context("no model for part1")?;
        let model_part2 = solver_part2.get_model().context("no model for part2")?;
        for x in xs.iter() {
            let res = model_part1.get_const_interp(x).unwrap().as_i64().unwrap();
            result_part1 += res;

            let res = model_part2.get_const_interp(x).unwrap().as_i64().unwrap();
            result_part2 += res;
        }
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}

fn parse_list<T: FromStr<Err: Into<anyhow::Error>>>(s: &str) -> Result<Vec<T>> {
    s.split(",")
        .map(|digit| digit.parse::<T>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn remove_braces(s: &str, open: char, close: char) -> Result<&str> {
    if let Some(s) = s.strip_prefix(open)
        && let Some(s) = s.strip_suffix(close)
    {
        Ok(s)
    } else {
        anyhow::bail!("expected '{open}' and '{close}'")
    }
}
