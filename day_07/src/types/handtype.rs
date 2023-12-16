use std::collections::HashMap;
use crate::types::card::Strength;
use crate::types::card::Card;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum HandType {
    FIVE,
    FOUR,
    FULL,
    THREE,
    PAIRS,
    TWO,
    HIGH
}

impl Strength for HandType {
    fn strength(&self) -> u8 {
        match self {
            HandType::FIVE => 6,
            HandType::FOUR => 5,
            HandType::FULL => 4,
            HandType::THREE => 3,
            HandType::PAIRS => 2,
            HandType::TWO => 1,
            HandType::HIGH => 0,
        } 
    }
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let mut char_counts = HashMap::new();
        
        // Iterate over each character in the string
        for c in value.chars() {
            // Update the count in the HashMap
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }

        // Count the pairs
        let mut has_three: bool = false;
        let mut two: Vec<Card> = vec![];

        // Iterate through all key-value pairs in the HashMap
        for (key, value) in char_counts.iter() {
            if *value == 5 { return HandType::FIVE }
            if *value == 4 { return HandType::FOUR }
            if *value == 3 { has_three = true }
            if *value == 2 { two.push(Card::from(*key)) }
        }

        // Check for FULL THREE PAIR TWO
        if has_three && two.len() == 1 { return HandType::FULL }
        if has_three && two.len() == 0 { return HandType::THREE }
        if two.len() == 2 { return HandType::PAIRS }
        if two.len() == 1 { return HandType::TWO }

        return HandType::HIGH;
    }
}