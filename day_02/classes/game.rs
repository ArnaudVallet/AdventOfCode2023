use std::cmp;
use crate::classes::set::Set;

#[derive(Debug)]
pub struct Game {
    id: i32,
    #[allow(dead_code)] sets: Vec<Set>,
    max_red: u8,
    max_green: u8,
    max_blue: u8,
}

impl Game {
    pub fn new(line: &str) -> Game {
        let game_sets = Game::sets_from(line);
        let (max_red, max_green, max_blue) = Game::max_colors_from(&game_sets);
        Game {
            id: Game::id_from(line),
            sets: game_sets,
            max_red,
            max_green,
            max_blue,
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
        let mut output_sets: Vec<Set> = Vec::new();
        let colon_index = input.find(':').unwrap();
        let filtered = &input[colon_index + 1..];
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

    pub(crate) fn max_colors_from(sets: &Vec<Set>) -> (u8, u8, u8) {
        // Calculate the highest for each color
        let (best_red, best_green, best_blue) = sets.iter()
            .fold((0, 0, 0), |acc, set| {
            (
                cmp::max(acc.0, set.get_red()), 
                cmp::max(acc.1, set.get_green()), 
                cmp::max(acc.2, set.get_blue())
            )
        });
        
        (best_red, best_green, best_blue)
    }

    pub fn is_possible(&self, red_init: u8, green_init: u8, blue_init: u8) -> bool {
        !(self.max_red > red_init
        || self.max_green > green_init
        || self.max_blue > blue_init)
    }

    pub fn get_id(&self) -> i32 { self.id }

    pub fn get_set_power(&self) -> i32 {
        self.max_red as i32 
        * self.max_green as i32 
        * self.max_blue as i32
    }
}
