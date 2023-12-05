use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Number {
    value: String,
    row_index: u8,
    start_index: u8, // inclusive
    end_index: u8,   // uninclusive
    is_adjacent: bool,
}

impl Number {
    pub fn new(value: &str, row_index: u8, start_index: u8, end_index: u8) -> Number {
        Number {
            value: value.to_string(),
            row_index,
            start_index,
            end_index,
            is_adjacent: false,
        }
    }

    pub fn get_value(&self) -> String {
        self.value.to_string()
    }

    pub fn get_start_index(&self) -> u8 {
        self.start_index
    }

    pub fn get_end_index(&self) -> u8 {
        self.end_index
    }

    pub fn get_range(&self) -> Range<usize> {
        let start = self.start_index.saturating_sub(1);
        let end = self.end_index.saturating_add(1).min(140);
        start as usize..end as usize
    }

    pub fn set_is_adjacent(mut self, is_adjacent: bool) {
        self.is_adjacent = is_adjacent;
    }
}
