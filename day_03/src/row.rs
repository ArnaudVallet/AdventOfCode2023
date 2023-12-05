use crate::number::Number;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Row {
    value: String,
    index: u8,
    numbers: Vec<Number>,
}

impl Row {
    pub fn new(input: String, row_index: u8) -> Row {
        Row {
            index: row_index,
            numbers: Row::numbers_from(&input, row_index),
            value: input,
        }
    }

    pub(crate) fn numbers_from(input: &str, row_index: u8) -> Vec<Number> {
        let mut numbers: Vec<Number> = Vec::new();
        // Define a regular expression pattern to match numeric substrings
        let pattern = Regex::new(r"\d+").unwrap();

        // Find all matches and iterate over them
        for mat in pattern.find_iter(input) {
            let start_index = mat.start() as u8;
            let end_index = mat.end() as u8;
            let numeric_substring = mat.as_str();
            numbers.push(Number::new(
                numeric_substring,
                row_index,
                start_index,
                end_index,
            ));
        }
        numbers
    }

    pub fn get_value(self) -> String {
        self.value
    }

    pub fn get_numbers(self) -> Vec<Number> {
        self.numbers
    }

    pub fn get_index(&self) -> u8 {
        self.index
    }
}
