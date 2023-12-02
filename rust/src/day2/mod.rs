use std::{path::Path, fs::{File, read_to_string}};

enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32)
}

impl From<&str> for Cube {
    fn from(s: &str) -> Self {
        let mut splitter = s.split_whitespace();
        let quantity = splitter.next().unwrap().parse::<u32>().unwrap();
        let color = splitter.next().unwrap();

        match color {
            "red" => Self::Red(quantity),
            "green" => Self::Green(quantity),
            "blue" => Self::Blue(quantity),
            _ => panic!("Invalid color")
        }
    }
}

struct Games {
    valid_games_sum: u32, 
    fewest_number_sum: u32,
}

impl From <String> for Games {
    fn from(s: String) -> Self {
        let mut valid_games: Vec<u32> = vec![];
        let mut fewest_number_sum: u32 = 0;
        let configuration = (12, 13, 14);

        for line in s.lines() {
            let mut splitter = line.split(":");
            let game_id = splitter.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
            let games = splitter.next().unwrap();
            let mut max_each: (u32, u32, u32) = (0, 0, 0);

            let valid_game = {
                let mut splitter = games.split(";");
                let mut games_result: bool = true;

                while let Some(game) = splitter.next() {
                    let mut cubes: Vec<Cube> = vec![];
                    let mut splitter = game.split(",");

                    while let Some(cube) = splitter.next() {
                        cubes.push(Cube::from(cube));
                    }

                    cubes.iter().for_each(|cube| {
                        match cube {
                            Cube::Red(quantity) => {
                                if *quantity > configuration.0 {
                                    games_result = false;
                                }

                                if *quantity > max_each.0 {
                                    max_each.0 = *quantity;
                                }
                            },
                            Cube::Green(quantity) => {
                                if *quantity > configuration.1 {
                                    games_result = false;
                                }

                                if *quantity > max_each.1 {
                                    max_each.1 = *quantity;
                                }
                            },
                            Cube::Blue(quantity) => {
                                if *quantity > configuration.2 {
                                    games_result = false;
                                }

                                if *quantity > max_each.2 {
                                    max_each.2 = *quantity;
                                }
                            }
                        }
                    });
                }

                games_result
            };

            if valid_game {
                valid_games.push(game_id);
            }

            let power = max_each.0 * max_each.1 * max_each.2;
            fewest_number_sum += power;
        }

        Self {
            valid_games_sum: valid_games.iter().sum(),
            fewest_number_sum
        }
    }
}

fn get_possible_games() -> (u32, u32) {
    let path = Path::new("src/day2/games.txt");
    let file = read_to_string(path).expect("Could not open file");

    let games = Games::from(file);

    (games.valid_games_sum, games.fewest_number_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_games() {
        assert_eq!(get_possible_games().0, 2406);
        assert_eq!(get_possible_games().1, 78375);
    }
}
