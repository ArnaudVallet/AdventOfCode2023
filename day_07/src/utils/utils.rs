use crate::types::hand::Hand;
use crate::types::card::{Card, EnumIterator, Strength};
use crate::types::handtype::HandType;

pub fn cards_from(string: &str) -> Vec<Card> {
    let parts = string.chars();
    parts.map(|char| Card::from(char)).collect()
}

pub fn hand_from(string: &str) -> Hand {
    let parts: Vec<&str> = string.split_whitespace().collect();
    let cards_str = parts[0];
    let bid = parts[1];
    Hand {
        bid: bid.parse::<u32>().unwrap(),
        rank: 0,
        cards: cards_from(cards_str),
        type_: HandType::from(cards_str),
        joker_type: best_handtype_with_joker_from(cards_str)
    }
}

pub fn best_handtype_with_joker_from(cards_str: &str) -> HandType {
    let mut final_handtype: HandType = HandType::HIGH;
    for card in Card::variants() {
        let card_char: char = card.into();
        let joker_replaced: &str = &cards_str.replace('J', &card_char.to_string());
        println!("From {} to {}", cards_str, joker_replaced);
        let found_type = HandType::from(joker_replaced);
        if found_type.strength() > final_handtype.strength() {
            final_handtype = found_type;
        }
    }
    println!("------------------");
    final_handtype
}