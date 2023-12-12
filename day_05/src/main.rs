mod types;

use std::collections::HashMap;
use std::fs;
use types::almanac::Almanac;

fn main() {
    let mut almanac = Almanac {
        maps: HashMap::new(),
        seeds: vec![],
    };

    let file_path = "light-input.txt";

    // Read the content of the file
    if let Ok(file_content) = fs::read_to_string(file_path) {
        // Split the text into sections based on empty lines
        let sections: Vec<&str> = file_content.split("\n\n").collect();

        // Print each section
        for (index, section) in sections.iter().enumerate() {
            println!("\x1b[1;32mSection {index}\x1b[0m: \n{}", section);

            if section.contains("map:") {
                almanac.add_map_from(section.to_string());
            } else if section.contains("seeds:") {
                almanac.add_seeds_from(section.to_string());
            }
        }
    } else {
        println!("Error reading the file");
    }

    let result1 = almanac
        .seeds
        .iter()
        .map(|seed| almanac.find_location_from(*seed))
        .min()
        .unwrap();

    let seeds = almanac.get_seeds_from_pairs();
    println!("seeds: {:#?}", seeds);
    let result2 = seeds
        .iter()
        .map(|seed| almanac.find_location_from(*seed))
        .min()
        .unwrap();

    println!("Result 1: {}", result1);

    println!("Result 2: {}", result2); // 219 974 726
}
