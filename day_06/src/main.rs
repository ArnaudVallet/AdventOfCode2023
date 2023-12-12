mod types;

use std::fs::File;
use std::io::{self, BufRead};

use types::race::Race;

fn parse_nums(line: String, to_trim: &str) -> Vec<u64> {
    line.trim_start_matches(to_trim)
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() -> io::Result<()> {
    
    // Open the file
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    // Times & Distances
    let mut times: Vec<u64> = vec![];
    let mut distances: Vec<u64> = vec![];
    let mut races: Vec<Race> = vec![];

    for line in reader.lines() {
        let line = line?;
        println!("line: {:?}", line);

        if line.contains("Time:") {
            let numbers: Vec<u64> = parse_nums(line, "Time:");
            times.extend(numbers);
        } else if line.contains("Distance:") {
            let numbers: Vec<u64> = parse_nums(line, "Distance:");
            distances.extend(numbers);
        }
    }

    // Define Races
    for i in 0..times.len() {
        races.push(Race{
            time: times[i],
            record: distances[i],
            runs: vec![]
        })
    }

    // Evaluate each possible run for each Race
        // let _ = races.iter_mut().map(|r| r.evaluate_each_run());
    for i in 0..races.len() {
        races[i].evaluate_each_run();
    }

    // Find better runs
    let result1 = races.iter().fold(1, |acc, r| {
        acc * r.find_better_runs().len()
    });
    println!("Result 1: {:#?}", result1);

    // Find the real solo Race Time and Record
    let final_race_time = times.iter().fold(String::new(), |acc, digit| {
        acc + &digit.to_string()
    }).parse::<u64>().unwrap();
    let final_race_record = distances.iter().fold(String::new(), |acc, digit| {
        acc + &digit.to_string()
    }).parse::<u64>().unwrap();
    let mut final_race = Race{
        time: final_race_time, 
        record: final_race_record,
        runs: vec![]
    };
    final_race.evaluate_each_run();
    let result2 = final_race.find_better_runs().len();
    println!("Result 2: {:#?}", result2);

    Ok(())
}
