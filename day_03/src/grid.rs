use crate::gear::Gear;
use crate::row::Row;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Grid {
    rows: Vec<Row>,
    adjacents_sum: i32,
    gears_ratio: i128,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            rows: Vec::new(),
            adjacents_sum: 0,
            gears_ratio: 0,
        }
    }

    pub fn add_row(&mut self, input: String, row_index: u8) {
        self.rows.push(Row::new(input, row_index));
    }

    pub fn get_adjacents_sum(self) -> i32 {
        self.adjacents_sum
    }

    pub fn get_gears_ratio(self) -> i128 {
        self.gears_ratio
    }

    pub fn find_adjacents(&mut self) {
        let num_and_colon = Regex::new(r"[^0-9.]").unwrap();

        for i in 0..self.rows.len() {
            let actual_row = &self.rows[i];
            let previous_row = if i > 0 { Some(&self.rows[i - 1]) } else { None };
            let next_row = self.rows.get(i + 1).clone();

            for number in actual_row.clone().get_numbers() {
                let mut is_adjacent: bool = false;

                // Previous row
                if let Some(previous) = previous_row {
                    let row_string = previous.clone().get_value();
                    let substring_bytes = &row_string.as_bytes()[number.get_range()];
                    let substring = std::str::from_utf8(substring_bytes).unwrap_or("");

                    if num_and_colon.is_match(substring) {
                        is_adjacent = true;
                    }
                }

                // Next row
                if !is_adjacent {
                    if let Some(next) = next_row {
                        let row_string = next.clone().get_value();
                        let substring_bytes = &row_string.as_bytes()[number.get_range()];
                        let substring = std::str::from_utf8(substring_bytes).unwrap_or("");

                        if num_and_colon.is_match(substring) {
                            is_adjacent = true;
                        }
                    }
                }

                // Actual row
                if !is_adjacent {
                    let row_string = actual_row.clone().get_value();
                    let substring_bytes = &row_string.as_bytes()[number.get_range()];
                    let substring = std::str::from_utf8(substring_bytes).unwrap_or("");

                    if num_and_colon.is_match(substring) {
                        is_adjacent = true;
                    }
                }

                if is_adjacent {
                    self.adjacents_sum += number.clone().get_value().parse::<i32>().unwrap();
                }
                number.set_is_adjacent(is_adjacent);
            }
        }
    }

    pub fn find_gears(&mut self) {
        
        let mut gears: Vec<Gear> = vec![];
        // Iterate row
        for i in 0..self.rows.len() {
            let actual_row = &self.rows[i];
            // Iterate char in row
            for (char_index, c) in actual_row.clone().get_value().chars().enumerate() {
                if c == '*' {
                    gears.push(Gear::new(actual_row.clone().get_index(), char_index as u8));
                }
            }
        }

        for i in 0..gears.len() {
            let gear = &mut gears[i];
            let previous_row = &self.rows[gear.get_row_index() as usize - 1];
            let actual_row = &self.rows[gear.get_row_index() as usize];
            let next_row = &self.rows[gear.get_row_index() as usize + 1];

            gear.find_gear_adjacent_numbers_from(previous_row, actual_row, next_row);

            if gear.is_engine() {
                // Multiply all numbers from the gear overlaps
                let gear_ratio = gear.clone().get_overlaps().iter().fold(1, |acc, &x| acc * x);
                self.gears_ratio += gear_ratio as i128;
            }
        }
    }
}
