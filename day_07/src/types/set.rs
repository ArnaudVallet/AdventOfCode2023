use crate::types::hand::Hand;

use super::card::Strength;

#[derive(Debug)]
pub struct Set {
    pub hands: Vec<Hand>
}

impl Set {
    pub fn sort_hands(&mut self) {
        self.hands.sort_unstable_by_key(|hand| (
            hand.type_.strength(), // Sort by HandType
            hand.first(false), // Sort by 1st Card
            hand.second(false), // Sort by 2nd Card
            hand.third(false), // Sort by 3rd Card
            hand.forth(false), // Sort by 4th Card
            hand.fifth(false), // Sort by 5th Card
        ));
    }

    pub fn sort_hands_with_joker(&mut self) {
        self.hands.sort_unstable_by_key(|hand| (
            hand.joker_type.strength(), // Sort by HandType with Joker
            hand.first(true), // Sort by 1st Card
            hand.second(true), // Sort by 2nd Card
            hand.third(true), // Sort by 3rd Card
            hand.forth(true), // Sort by 4th Card
            hand.fifth(true), // Sort by 5th Card
        ));
    }
}