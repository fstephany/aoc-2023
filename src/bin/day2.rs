fn main() {
    let file_content = std::fs::read_to_string("inputs/day2").unwrap();
    println!("Part 1: Sum of valid Game IDs: {}", part_one(&file_content));
    println!("Part 2: Sum of game powers: {}", part_two(&file_content));
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

            for cube_draw in draw.split(',') {
                // example: " 4 blue"
                let mut splitted = cube_draw.trim().split_whitespace();
                let number = splitted.next().unwrap().parse::<u64>().unwrap();
                let color = splitted.next().unwrap();

                let valid_draw = match color {
                    "red" if number > 12 => false,
                    "blue" if number > 14 => false,
                    "green" if number > 13 => false,
                    _ => true, /* draw is legit OR we are dealing with an invalid input */
                };

                if !valid_draw {
                    is_game_valid = false;
                    break; // do not bother to check the other draws.
                }
            }
        }

        if is_game_valid {
            valid_games.push(game_id)
        }
    }

    valid_games.iter().sum()
}

fn part_two(input: &str) -> usize {
    let mut game_powers: Vec<usize> = Vec::new();

    for game in input.lines() {
        let game_data = game.rsplit(':').next().unwrap();
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for draw in game_data.split(';') {
            // Draw example:
            // "3 green, 4 blue, 1 red"
            // "1 blue, 2 green"
            for cube_draw in draw.split(',') {
                // example: " 4 blue"
                let mut splitted = cube_draw.trim().split_whitespace();
                let number = splitted.next().unwrap().parse::<usize>().unwrap();
                let color = splitted.next().unwrap();

                match color {
                    "red" if number > min_red => min_red = number,
                    "blue" if number > min_blue => min_blue = number,
                    "green" if number > min_green => min_green = number,
                    _ => (), /* beware that this also globs invalid input  */
                }
            }
        }

        game_powers.push(min_red * min_blue * min_green);
    }

    game_powers.iter().sum()
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

    #[test]
    fn example_part_2() {
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

        assert_eq!(part_two(input), 2286);
    }

    #[test]
    fn real_part_1() {
        let input = std::fs::read_to_string("inputs/day2").unwrap();
        assert_eq!(part_one(&input), 2727);
    }

    #[test]
    fn real_part_2() {
        let input = std::fs::read_to_string("inputs/day2").unwrap();
        assert_eq!(part_two(&input), 56580);
    }
}
