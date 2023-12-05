#[derive(Debug, Clone)]
pub struct Card {
    id: u8,
    numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
    common_numbers: Vec<u8>,
    points: i32, // Part 1
    quantity: u32, // Part 2
}

impl Card {
    pub fn new(id: u8, input: String) -> Card {
        let c = Card {
            id,
            numbers: Card::numbers_from(input.to_string()),
            winning_numbers: Card::winning_numbers_from(input.to_string()),
            common_numbers: vec![],
            points: 0,
            quantity: 1
        };
        c.find_common_numbers().calculate_score()
        
    }

    pub fn numbers_from(input: String) -> Vec<u8> {
        let start = input.find(":").unwrap();
        let end = input.find("|").unwrap();
        let substring = input[start..end].trim();
        substring.split_whitespace()
            .map(|n| n.parse::<u8>())
            .filter_map(Result::ok)
            .collect()
    }

    pub fn winning_numbers_from(input: String) -> Vec<u8> {
        let start = input.find("|").unwrap();
        let substring = input[start..].trim();
        substring.split_whitespace()
            .map(|n| n.parse::<u8>())
            .filter_map(Result::ok)
            .collect()
    }

    pub fn find_common_numbers(mut self) -> Self {
        self.common_numbers = self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .cloned()
            .collect();
        self
    }

    pub fn calculate_score(mut self) -> Card {
        self.points = self.common_numbers
            .iter()
            .fold(0, |acc, &_x| {
                if self.common_numbers.is_empty() {
                    acc
                } else if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            });
        self
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_points(&self) -> i32 {
        self.points
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn add_quantity(&mut self, amount: u32) {
        self.quantity += amount
    }
 
    pub fn get_common_numbers_count(&self) -> usize {
        self.common_numbers.len()
    }
}