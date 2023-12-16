mod types;
mod utils;

use std::fs;

use types::card::Card;
use types::hand::Hand;
use types::set::Set;
// use crate::types::card::Strength;
use utils::utils::hand_from;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // States
    let mut set = Set{ hands: vec![] };
    let mut hands: Vec<Hand> = vec![];

    // Read the entire file into a string
    let file_content = fs::read_to_string("input.txt")?;

    // Split the string into lines
    for line in file_content.lines() {
        hands.push(hand_from(line));
    }
    set.hands = hands;

    // Part 1
    set.sort_hands();
    for i in 1..=set.hands.len() {
        set.hands[i-1].rank = i as u32;
        println!("{:?} has rank {} !", set.hands[i-1], i);
    }
    let result1 = set.hands.iter().fold(0, |acc, h| {
        acc + h.bid * h.rank
    });
    println!("Result 1: {}", result1);

    // Part 2
    set.sort_hands_with_joker();
    for i in 1..=set.hands.len() {
        set.hands[i-1].rank = i as u32;
        println!("{:?} has rank {} !", set.hands[i-1], i);
    }
    let result2 = set.hands.iter().fold(0, |acc, h| {
        acc + h.bid * h.rank
    });
    println!("Result 2: {}", result2);

    Ok(())
}
