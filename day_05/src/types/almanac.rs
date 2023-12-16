use crate::types::converter::Converter;
use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
pub struct Almanac {
    pub maps: HashMap<String, Vec<Converter>>,
    pub seeds: Vec<u128>,
}

impl Almanac {
    pub fn add_map_from(&mut self, input: String) {
        let map_title = input.split("map:").next().unwrap().trim().to_string();
        let map_rows = input.splitn(2, "map:").nth(1).unwrap();
        let to_insert = Almanac::create_converters_from(map_rows.to_string());
        self.maps.insert(map_title, to_insert);
    }

    pub fn create_converters_from(rows: String) -> Vec<Converter> {
        let mut converters: Vec<Converter> = vec![];
        let lines: Vec<&str> = rows.lines().filter(|line| !line.is_empty()).collect();
        for line in lines {
            let splitted_line: Vec<&str> = line.split(' ').collect();
            let destination_start = splitted_line[0].parse::<u128>().unwrap();
            let source_start = splitted_line[1].parse::<u128>().unwrap();
            let range = splitted_line[2].parse::<u128>().unwrap();

            converters.push(Converter {
                destination_start,
                source_start,
                range,
            });
        }
        converters
    }

    pub fn add_seeds_from(&mut self, input: String) {
        let numbers: Vec<u128> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        self.seeds = numbers;
    }

    pub fn find_location_from(&self, seed: u128) -> u128 {
        let soil = self.map_from("seed-to-soil".to_string(), seed);
        let fertilizer = self.map_from("soil-to-fertilizer".to_string(), soil);
        let water = self.map_from("fertilizer-to-water".to_string(), fertilizer);
        let light = self.map_from("water-to-light".to_string(), water);
        let temperature = self.map_from("light-to-temperature".to_string(), light);
        let humidity = self.map_from("temperature-to-humidity".to_string(), temperature);
        let location = self.map_from("humidity-to-location".to_string(), humidity);
        location
    }

    pub fn map_from(&self, map_name: String, source: u128) -> u128 {
        let mapper = self
            .maps
            .get(&map_name)
            .unwrap()
            .iter()
            .filter(|&converter| converter.is_in_range(source))
            .next();
        match mapper {
            Some(converter) => converter.destination_from(source),
            None => source,
        }
    }

    pub fn get_seeds_from_pairs(&self) -> Vec<Vec<u128>> {
        let mut final_seeds_ranges = vec![];
        let mut actual_seeds = self.seeds.clone();
        while actual_seeds.len() >= 2 {
            let start_and_end: Vec<u128> = actual_seeds.drain(0..2).collect();
            let range: Vec<u128> = vec![start_and_end[0], start_and_end[0] + start_and_end[1]];
            final_seeds_ranges.push(range);
        }
        final_seeds_ranges
    }
}
