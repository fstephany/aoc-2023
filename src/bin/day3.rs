fn main() {
    let file_content = std::fs::read_to_string("inputs/day3").unwrap();
    println!("Part 1: sum of part numbers: {}", part_one(&file_content));
}

// struct

fn part_one(input: &str) -> u64 {
    let width = input.find(char::is_whitespace).unwrap(); // all lines have the same width
    let input: String = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    let symbol_indices: Vec<usize> = input
        .match_indices(|c: char| !(c.is_ascii_digit() || c == '.'))
        .map(|m| m.0)
        .collect();

    let mut correct_parts: Vec<u64> = Vec::new();

    let mut in_number = false;
    let mut current_number = String::new();
    let mut has_adjacent_symbol = false;

    for (i, char) in input.chars().enumerate() {
        let mut process_end_of_number = || {
            if in_number && has_adjacent_symbol {
                let parsed = current_number.parse().unwrap();
                // println!("Found a correct part number: {parsed} (idx: {i})");
                correct_parts.push(parsed);
            }

            // Reset
            in_number = false;
            current_number = String::new();
            has_adjacent_symbol = false;
        };

        let check_top = i >= width;
        let check_bottom = i < input.len() - width;
        let check_left = (i % width) != 0; //
        let check_right = (i % width) != width - 1;

        // If we are the beginning of a line, we process the remaining of the
        // previous line. Because we might have a number ending a line.
        if i % width == 0 {
            process_end_of_number()
        }

        if char.is_ascii_digit() {
            in_number = true;
            current_number.push(char);
            if !has_adjacent_symbol {
                has_adjacent_symbol = (check_top
                    && check_left
                    && symbol_indices.contains(&(i - width - 1)))
                    || (check_top && symbol_indices.contains(&(i - width)))
                    || (check_top && check_right && symbol_indices.contains(&(i - width + 1)))
                    || (check_left && symbol_indices.contains(&(i - 1)))
                    || (check_right && symbol_indices.contains(&(i + 1)))
                    || (check_bottom && check_left && symbol_indices.contains(&(i + width - 1)))
                    || (check_bottom && symbol_indices.contains(&(i + width)))
                    || (check_bottom && check_right && symbol_indices.contains(&(i + width + 1)))
            }
        } else {
            process_end_of_number()
        }
    }

    // If we are on the last element and were parsing a number.
    if in_number && has_adjacent_symbol {
        // println!("Found a correct part number: {current_number}");
        correct_parts.push(current_number.parse().unwrap());
    }

    println!("Found {} correct parts", correct_parts.len());
    correct_parts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = r#"467..114..
                       ...*......
                       ..35..633.
                       ......#...
                       617*......
                       .....+.58.
                       ..592.....
                       ......755.
                       ...$.*....
                       .664.598.."#;

        assert_eq!(part_one(input), 4361);
    }

    #[test]
    fn consider_last_number() {
        let input = r#"....*...
                       .....935"#;

        assert_eq!(part_one(input), 935);
    }

    #[test]
    fn split_numbers_on_multiple_lines() {
        let input = r#"...*4
                       7...."#;

        assert_eq!(part_one(input), 4);
    }
}
