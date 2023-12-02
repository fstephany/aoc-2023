use std::fs;

fn main() {
    let file_content = fs::read_to_string("inputs/day1").unwrap();
    let total: u64 = file_content
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let assembled = String::from_iter([first_digit, last_digit]);
            assembled.parse::<u64>().unwrap()
        })
        .sum();

    println!("Sum of all the calibration values: {total}");
}
