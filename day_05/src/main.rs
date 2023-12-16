mod types;

use std::cmp::min;
use std::collections::HashMap;
use std::f64::INFINITY;
use std::fs;
use types::almanac::Almanac;

fn main() {
    let mut almanac = Almanac {
        maps: HashMap::new(),
        seeds: vec![],
    };

    let file_path = "input.txt";

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

    let seeds_ranges = almanac.get_seeds_from_pairs();
    println!("seeds_ranges: {:#?}", seeds_ranges);

    // For loop on ranges
    let mut result2: u128 = INFINITY as u128; 
    for i in 0..seeds_ranges.len() {
        let range = &seeds_ranges[i];
        let start = range[0];
        let end = range[1];
        for seed_num in start..end {
            let location = almanac.find_location_from(seed_num);
            result2 = min(location, result2);
        }
    }

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2); // This will take time... 2h30 for me ¯\_(ツ)_/¯
}
