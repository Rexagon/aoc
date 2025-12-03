use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

pub fn run() -> Result<()> {
    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for line in INPUT.lines() {
        result_part1 += compute_bank_voltage(line.trim(), 2);
        result_part2 += compute_bank_voltage(line.trim(), 12);
    }

    println!("part1: {result_part1}");
    println!("part1: {result_part2}");
    Ok(())
}

fn compute_bank_voltage(digits: &str, n: usize) -> u64 {
    let digits = digits.trim().as_bytes();

    let mut result = 0u64;
    let mut remaining_digits = n;

    let mut window_start = 0;
    let mut window_end = digits.len() - n + 1;
    while remaining_digits > 0 && window_end <= digits.len() {
        let mut max_digit_pos = window_start;
        let mut max_digit = digit(digits[window_start]);

        for i in window_start..window_end {
            let digit = digit(digits[i]);
            if digit > max_digit {
                max_digit = digit;
                max_digit_pos = i;
            }
        }

        remaining_digits -= 1;
        result += (max_digit as u64) * 10u64.pow(remaining_digits as u32);

        window_start = max_digit_pos + 1;
        window_end += 1;
    }
    assert_eq!(remaining_digits, 0);

    result
}

fn digit(char: u8) -> u8 {
    char - b'0'
}
