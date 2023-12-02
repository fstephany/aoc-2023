fn main() {
    let file_content = std::fs::read_to_string("inputs/day2").unwrap();
    println!("Part 1: Sum of valid Game IDs: {}", part_one(&file_content));
}

fn part_one(input: &str) -> usize {
    let mut valid_games: Vec<usize> = Vec::new();

    for (index, game) in input.lines().enumerate() {
        let game_id = index + 1;
        let game_data = game.rsplit(':').next().unwrap();

        let mut is_game_valid = true;

        for draw in game_data.split(';') {
            // Draw example:
            // "3 green, 4 blue, 1 red"
            // "1 blue, 2 green"
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for cube_draw in draw.split(',') {
                let mut splitted = cube_draw.trim().split_whitespace();
                let number = splitted.next().unwrap().parse::<u64>().unwrap();
                let color = splitted.next().unwrap();

                match color {
                    "red" => red = number,
                    "blue" => blue = number,
                    "green" => green = number,
                    _ => (), /* Do not handle invalid input */
                }
            }

            if red > 12 || green > 13 || blue > 14 {
                is_game_valid = false;
                // No need to check the other draws, the game is already invalid
                break;
            }
        }

        if is_game_valid {
            valid_games.push(game_id)
        }
    }

    valid_games.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        // The Elf would first like to know which games would have been possible
        // if the bag contained only 12 red cubes, 13 green cubes, and 14 blue
        // cubes?

        // In the example above, games 1, 2, and 5 would have been possible if
        // the bag had been loaded with that configuration. However, game 3
        // would have been impossible because at one point the Elf showed you 20
        // red cubes at once; similarly, game 4 would also have been impossible
        // because the Elf showed you 15 blue cubes at once. If you add up the
        // IDs of the games that would have been possible, you get 8.

        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(part_one(input), 8);
    }
}
