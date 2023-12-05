mod types;

use std::fs::File;
use std::io::{self, BufRead};
use types::card::Card;

fn main() -> io::Result<()> {

    // Open the file in read-only mode
    let file = File::open("input.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut card_id: u8 = 1;
    let mut cards: Vec<Card> = vec![];
    for line_result in reader.lines() {
        // Handle each line or any potential errors
        match line_result {
            Ok(line) => {
                cards.push(Card::new(card_id, line));
                // cards_id_quantity_map.insert(card_id, 1);
                card_id += 1;
            },
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    for card_index in 0..cards.len() {
        let wins = cards[card_index].get_common_numbers_count();
        let quantity = cards[card_index].get_quantity();
        let start = cards[card_index].get_id();
        let end = cards[card_index].get_id() + wins as u8;

        // Iterate on following cards as many times as actual card has wins
        for i in start..end {
            // Update the quantity for every subsequent Card by the quantity of the actual Card
            if i < cards.len() as u8 { // Prevent going out of bounds
                cards[i as usize].add_quantity(quantity);
            }
        }
    }

    let result1 = cards.iter().clone().fold(0, |acc, card| {
        acc + card.get_points()
    });

    let result2 = cards.iter().fold(0, |acc, card| {
        acc + card.get_quantity()
    });

    println!("[Part1] Result : {:#?}", result1);
    println!("[Part2] Result : {:#?}", result2);

    Ok(())
}
