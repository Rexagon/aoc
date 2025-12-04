use std::str::FromStr;

use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut map = INPUT.parse::<Map>()?;

    let mut to_remove = Vec::new();
    map.find_accessible(&mut to_remove);
    let result_part1 = to_remove.len();

    let mut result_part2 = 0;
    while !to_remove.is_empty() {
        to_remove.clear();
        map.find_accessible(&mut to_remove);
        map.remove_accessible(&to_remove);
        result_part2 += to_remove.len();
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}

struct Map {
    items: Vec<bool>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0usize;
        let mut items = Vec::new();
        for line in s.trim().lines() {
            let mut line_width = 0;
            for char in line.trim().chars() {
                items.push(char == '@');
                line_width += 1;
            }
            anyhow::ensure!(width == 0 || line_width == width, "invalid line");
            width = line_width;
        }
        let height = items.len() / width;

        Ok(Self {
            items,
            width,
            height,
        })
    }
}

impl Map {
    fn find_accessible(&self, output: &mut Vec<usize>) {
        const LIMIT: usize = 4;

        for (pos, is_roll) in self.items.iter().copied().enumerate() {
            if !is_roll {
                continue;
            }

            let pos_x = pos % self.width;
            let pos_y = pos / self.width;

            let mut rolls_around = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0
                        || !check_bounds(pos_x, x, self.width)
                        || !check_bounds(pos_y, y, self.height)
                    {
                        continue;
                    }

                    let pos_to_check = pos as isize + x + y * self.width as isize;
                    let has_roll = self
                        .items
                        .get(pos_to_check as usize)
                        .copied()
                        .unwrap_or_default();
                    rolls_around += has_roll as usize;
                }
            }

            if rolls_around < LIMIT {
                output.push(pos);
            }
        }
    }

    fn remove_accessible(&mut self, to_remove: &[usize]) {
        for pos in to_remove {
            self.items[*pos] = false;
        }
    }
}

fn check_bounds(value: usize, diff: isize, max: usize) -> bool {
    let value = value as isize + diff;
    value >= 0 && value < max as isize
}
