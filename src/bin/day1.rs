fn main() {
    let file_content = std::fs::read_to_string("inputs/day1").unwrap();

    println!(
        "Part 1: Sum of all the calibration values: {}",
        part_one(&file_content)
    );

    println!(
        "Part 2: Sum of all the calibration values: {}",
        part_two_quick_and_dirty(&file_content)
    );
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
            let assembled = String::from_iter([first_digit, last_digit]);
            assembled.parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

/// In which the inner computer science guy in me dies.
fn part_two_quick_and_dirty(input: &str) -> u64 {
    // We keep the first and last letter when we replace. This is useful for
    // numbers that overlap (e.g., in "eightwo", the 't' is used by both numbers)
    let file_content = input
        .to_string()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    file_content
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
            let assembled = String::from_iter([first_digit, last_digit]);
            assembled.parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        // In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
        // Adding these together produces 281.
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        assert_eq!(part_one(input), 142);
    }

    #[test]
    fn example_part_2() {
        // In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
        // Adding these together produces 281.
        let input = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#;
        assert_eq!(part_two_quick_and_dirty(input), 281);
    }

    #[test]
    fn real_part_1() {
        let input = std::fs::read_to_string("inputs/day1").unwrap();
        assert_eq!(part_one(&input), 54159);
    }

    #[test]
    fn real_part_2() {
        let input = std::fs::read_to_string("inputs/day1").unwrap();
        assert_eq!(part_two_quick_and_dirty(&input), 53866);
    }
}
