use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut coords = Vec::new();
    for line in INPUT.lines() {
        let mut line = line.split(",").map(str::parse::<i64>);
        let x = line.next().unwrap()?;
        let y = line.next().unwrap()?;
        let z = line.next().unwrap()?;
        coords.push(((x, y, z), 0));
    }

    let n = coords.len();

    let mut distances = Vec::with_capacity((n * n - n) / 2);
    for i in 0..n {
        for j in 0..i {
            let d = distance(coords[i].0, coords[j].0);
            distances.push((d, i, j));
        }
    }
    distances.sort_unstable_by(|(d1, ..), (d2, ..)| d1.cmp(d2));

    let mut circuits = Vec::<Vec<usize>>::new();
    let mut free_circuits = Vec::<u32>::new();

    let mut result_part1 = 1;
    let mut result_part2 = 0;
    for (pair_idx, &(_, i, j)) in distances.iter().enumerate() {
        match (coords[i].1, coords[j].1) {
            (0, 0) => {
                let id = free_circuits.pop().unwrap_or_else(|| {
                    circuits.push(Vec::<usize>::new());
                    circuits.len() as u32
                });
                coords[i].1 = id;
                coords[j].1 = id;
                circuits[id as usize - 1].push(i);
                circuits[id as usize - 1].push(j);
            }
            (0, id) => {
                coords[i].1 = id;
                circuits[id as usize - 1].push(i);
            }
            (id, 0) => {
                coords[j].1 = id;
                circuits[id as usize - 1].push(j);
            }
            (mut id1, mut id2) if id1 != id2 => {
                if id1 > id2 {
                    std::mem::swap(&mut id1, &mut id2);
                }

                let [c1, c2] = circuits
                    .get_disjoint_mut([id1 as usize - 1, id2 as usize - 1])
                    .unwrap();

                for &i in c2.as_slice() {
                    coords[i].1 = id1;
                }

                c1.extend(c2.drain(..));
                free_circuits.push(id2);

                coords[i].1 = id1;
                coords[j].1 = id1;
            }
            _ => {}
        }

        if pair_idx + 1 == n {
            let mut circuits = circuits.clone();
            circuits.sort_by(|a, b| b.len().cmp(&a.len()));
            for circuit in circuits.iter().take(3) {
                if circuit.is_empty() {
                    break;
                }
                result_part1 *= circuit.len();
            }
        }

        if circuits[coords[i].1 as usize - 1].len() == n {
            result_part2 = coords[i].0.0 * coords[j].0.0;
            break;
        }
    }

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}

fn distance(a: Coord, b: Coord) -> i64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let dz = b.2 - a.2;
    dx * dx + dy * dy + dz * dz
}

type Coord = (i64, i64, i64);
