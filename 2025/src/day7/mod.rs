use anyhow::Result;

static INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let (first_line, other_lines) = INPUT.split_once("\n").unwrap();
    let width = first_line.len();

    let start_index = first_line
        .as_bytes()
        .iter()
        .position(|c| *c == b'S')
        .unwrap();
    let mut beams = vec![0u64; width];
    beams[start_index] = 1;
    let mut next_beams = beams.clone();

    let mut result_part1 = 0usize;
    for line in other_lines.lines() {
        for (i, c) in line.trim().as_bytes().iter().enumerate() {
            if *c == b'^' && beams[i] > 0 {
                next_beams[i] = 0;
                next_beams[i - 1] += beams[i];
                next_beams[i + 1] += beams[i];
                result_part1 += 1;
            }
        }

        std::mem::swap(&mut beams, &mut next_beams);
        next_beams.clear();
        next_beams.extend_from_slice(&beams);
    }

    let result_part2 = next_beams.iter().sum::<u64>();

    println!("part1: {result_part1}");
    println!("part2: {result_part2}");
    Ok(())
}
