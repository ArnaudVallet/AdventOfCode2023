#[derive(Debug, Clone)]
pub struct Converter {
    pub destination_start: u128,
    pub source_start: u128,
    pub range: u128,
}

impl Converter {
    pub fn is_in_range(&self, input: u128) -> bool {
        self.source_start <= input && input < self.source_start + self.range
    }

    pub fn destination_from(&self, source: u128) -> u128 {
        source - self.source_start + self.destination_start
    }
}