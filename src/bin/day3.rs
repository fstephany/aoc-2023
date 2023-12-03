use std::collections::HashMap;

fn main() {
    let file_content = std::fs::read_to_string("inputs/day3").unwrap();
    println!("Part 1: sum of part numbers: {}", part_one(&file_content));
    println!("Part 2: sum of gear ratios: {}", part_two(&file_content));
}

/// The general idea is to work on a continuous stream of chars without trying
/// to be too smart.
///
/// 1. Remove all the whitespace (i.e. newlines) from the input
/// 2. Extract the position of all the symbols
/// 3. Go through the data one char at a time.
///     - a. Accumulate the consecutive digits that form a number
///     - b. Check for each digit the top-left, top, top-right, left, right,
///       bottom-left, bottom, bottom-right position to see if there is a symbol
///       around.
///     - c. Once we we reach the end of a number, check if it is a valid part
///       number (ie., it has an adjacent symbol).
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

        // If the current char is on a border of the board (first row, last row,
        // first col, last col), there's no point trying to look in certain
        // directions.
        let check_top = i >= width; // not first row
        let check_bottom = i < input.len() - width; // not last row
        let check_left = (i % width) != 0; // not first col
        let check_right = (i % width) != width - 1; // not last col

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

/// It almost follows the same principle as `part_one` but this time we are only
/// interested in '*' symbols.
///
/// Instead of just storing the position of a symbol, we also store the numbers
/// that are adjacent to it.
///
fn part_two(input: &str) -> u64 {
    let width = input.find(char::is_whitespace).unwrap();
    let input: String = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();

    // We keep a list of gears with:
    // - their position (key)
    // - the numbers that it touches (values)
    let mut gears: HashMap<usize, Vec<u64>> = input
        .match_indices(|c: char| c == '*')
        .map(|m| (m.0, Vec::new()))
        .collect();

    let mut in_number = false;
    let mut current_number = String::new();
    let mut adjacent_gear: Option<usize> = None;

    for (i, char) in input.chars().enumerate() {
        let mut process_end_of_number = || {
            if let Some(gear) = adjacent_gear {
                if in_number {
                    let parsed = current_number.parse().unwrap();
                    gears.get_mut(&gear).unwrap().push(parsed);
                }
            }

            // Reset
            in_number = false;
            current_number = String::new();
            adjacent_gear = None;
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
            if adjacent_gear.is_none() {
                // Check if the current char digit is touching a gear.
                // Beware that this doesn't work if the current number touches
                // multiple gear.
                adjacent_gear = if check_top && check_left && gears.contains_key(&(i - width - 1)) {
                    Some(i - width - 1)
                } else if check_top && gears.contains_key(&(i - width)) {
                    Some(i - width)
                } else if check_top && check_right && gears.contains_key(&(i - width + 1)) {
                    Some(i - width + 1)
                } else if check_left && gears.contains_key(&(i - 1)) {
                    Some(i - 1)
                } else if check_right && gears.contains_key(&(i + 1)) {
                    Some(i + 1)
                } else if check_bottom && check_left && gears.contains_key(&(i + width - 1)) {
                    Some(i + width - 1)
                } else if check_bottom && gears.contains_key(&(i + width)) {
                    Some(i + width)
                } else if check_bottom && check_right && gears.contains_key(&(i + width + 1)) {
                    Some(i + width + 1)
                } else {
                    None
                }
            }
        } else {
            process_end_of_number()
        }
    }

    // If we are on the last element and were parsing a number.
    if let Some(gear) = adjacent_gear {
        if in_number {
            let parsed = current_number.parse().unwrap();
            gears.get_mut(&gear).unwrap().push(parsed);
        }
    }

    gears
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|p| p.get(0).unwrap() * p.get(1).unwrap())
        .sum()
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

    #[test]
    fn example_part_2() {
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

        assert_eq!(part_two(input), 467835);
    }

    #[test]
    fn real_part_1() {
        let input = std::fs::read_to_string("inputs/day3").unwrap();
        assert_eq!(part_one(&input), 527369);
    }

    #[test]
    fn real_part_2() {
        let input = std::fs::read_to_string("inputs/day3").unwrap();
        assert_eq!(part_two(&input), 73074886);
    }
}
