use std::collections::HashSet;

fn main() {
    let file_content = std::fs::read_to_string("inputs/day4").unwrap();
    println!("Part 1: {}", part_one(&file_content));
    println!("Part 2: {}", part_two(&file_content));
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (_card_id, data) = line.split_once(": ").unwrap();
            let (left_numbers, right_numbers) = data.split_once("|").unwrap();

            let winning_numbers_hash = left_numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let drawed_number = right_numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let matching_count = winning_numbers_hash.intersection(&drawed_number).count();

            if matching_count > 0 {
                2u64.pow((matching_count - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

/// The idea is to update the `number_of_draws_per_card` after we draw a card.
fn part_two(input: &str) -> u64 {
    let mut number_of_draws_per_card = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let (_card_name, data) = line.split_once(": ").unwrap();
        let (left_numbers, right_numbers) = data.split_once("|").unwrap();
        let winning_numbers_hash = left_numbers
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let drawed_number = right_numbers
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let matching_count = winning_numbers_hash.intersection(&drawed_number).count();
        let copies_of_current_card = number_of_draws_per_card.get(i).unwrap();

        for _ in 0..*copies_of_current_card {
            // Create copies of following card based on the number of wins
            for j in 0..matching_count {
                let card_to_copy = i + j + 1;
                number_of_draws_per_card[card_to_copy] = number_of_draws_per_card[card_to_copy] + 1
            }
        }
    }

    number_of_draws_per_card.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn real_part_1() {
        let input = std::fs::read_to_string("inputs/day4").unwrap();
        assert_eq!(part_one(&input), 21568);
    }

    #[test]
    fn example_part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_two(input), 30);
    }

    #[test]
    fn real_part_2() {
        let input = std::fs::read_to_string("inputs/day4").unwrap();
        assert_eq!(part_two(&input), 11827296);
    }
}
