// game.rs
use crate::classes::set::Set;

#[derive(Debug)]
pub struct Game {
    id: i32,
    sets: Vec<Set>,
    max_red: u8,
    max_green: u8,
    max_blue: u8,
}

impl Game {
    pub fn new(line: &str) -> Game {
        Game {
            id: Game::id_from(line),
            sets: Game::sets_from(line),
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        }
    }

    pub(crate) fn id_from(input: &str) -> i32 {
        let colon_index = input.find(':').unwrap();
        let filtered = &input[..colon_index];
        let id_string = filtered.strip_prefix("Game ").unwrap();
        let id_int = id_string.parse::<i32>();
        id_int.unwrap()
    }

    pub(crate) fn sets_from(input: &str) -> Vec<Set> {
        println!("{}", input);
        let mut output_sets: Vec<Set> = Vec::new();
        let colon_index = input.find(':').unwrap();
        let filtered = &input[colon_index + 1..];
        println!("{}", filtered);
        // Split the string by ";"
        let sets: Vec<&str> = filtered.split(';').collect();

        for set in sets {
            let mut red: u8 = 0;
            let mut green: u8 = 0;
            let mut blue: u8 = 0;

            let colors: Vec<&str> = set.split(',').collect();
            for color in colors {
                let parts: Vec<&str> = color.trim().split_whitespace().collect();
                let color_name = *parts.get(1).unwrap();
                let color_value = *parts.get(0).unwrap();
                match color_name {
                    "red" => red = color_value.parse::<u8>().unwrap(),
                    "green" => green = color_value.parse::<u8>().unwrap(),
                    "blue" => blue = color_value.parse::<u8>().unwrap(),
                    _ => {}
                }
            }
            let new_set = Set::new(red, green, blue);
            output_sets.push(new_set);
        }
        output_sets
    }
}
