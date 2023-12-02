use std::{path::Path, fs::{File, read_to_string}, io::BufReader, io::BufRead};

struct GameConfiguration(u32, u32, u32);
struct Games {
    configuration: GameConfiguration,
    valid_games_sum: u32, 
}

impl GameConfiguration {
    fn new(a: u32, b: u32, c: u32) -> Self {
        Self(a, b, c)
    }

    fn is_valid(&self, a: u32, b: u32, c: u32) -> bool {
        true
    }
}

impl From <String> for Games {
    fn from(s: String) -> Self {
        let configuration = GameConfiguration::new(12, 13, 14);
        let valid_games: Vec<u32> = vec![];

        for line in s.lines() {
            let mut splitter = line.split(":");
            let game_id = splitter.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
            let games = splitter.next().unwrap();
            let valid_game = {
                let mut splitter = games.split(";");
                let games_result: Vec<bool> = vec![];

                while let Some(game) = splitter.next() {
                    println!("{:?}", game);
                }
            };

            println!("{:?}", game_id);
            println!("{:?}", games);
        }

        Self {
            configuration,
            valid_games_sum: 0,
        }
    }
}

fn get_possible_games() -> u32 {
    let path = Path::new("src/day2/games.txt");
    let file = read_to_string(path).expect("Could not open file");

    let games = Games::from(file);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_games() {
        assert_eq!(get_possible_games(), 0);
    }
}
