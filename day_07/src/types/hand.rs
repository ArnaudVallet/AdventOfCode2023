use crate::Card;
use crate::types::handtype::HandType;

use super::card::Strength;

#[derive(Debug)]
pub struct Hand {
    pub bid: u32,
    pub rank: u32,
    pub cards: Vec<Card>,
    pub type_: HandType, 
    pub joker_type: HandType,
}

impl Hand {
    pub fn first(&self, with_joker: bool) -> u8 {
        self.card_strength(0, with_joker)
    }
    pub fn second(&self, with_joker: bool) -> u8 {
        self.card_strength(1, with_joker)
    }
    pub fn third(&self, with_joker: bool) -> u8 {
        self.card_strength(2, with_joker)
    }
    pub fn forth(&self, with_joker: bool) -> u8 {
        self.card_strength(3, with_joker)
    }
    pub fn fifth(&self, with_joker: bool) -> u8 {
        self.card_strength(4, with_joker)
    }
    fn card_strength(&self, num: u8, with_joker: bool) -> u8 {
        let card_strength = self.cards[num as usize].strength();
        if with_joker && card_strength == 11 {
            Card::TWO.strength() - 1
        } else {
            self.cards[num as usize].strength()
        }
    }
}