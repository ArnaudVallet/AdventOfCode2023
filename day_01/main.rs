use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


fn from_str(s: &str) -> &str {
    match s.to_lowercase().as_str() {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "error"
    }
}

fn is_numeric(value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn reverse_string(input: String) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    let pattern = Regex::new(r"(zero|one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let pattern_reversed = Regex::new(r"(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

    let mut count = 0;

    // Read the file line by line
    for line in reader.lines() {
        let mut result: String = "".to_string();
        let txt = line.unwrap_or_else(|error| {
            eprintln!("Error reading file: {}", error);
            "default_value_for_error_case".to_string()
        });
        let txt_reversed = reverse_string(txt.clone());
        println!("txt_reversed : {}", txt_reversed);
        
        if let Some(matched) = pattern.find(&txt) {
            let start = matched.start();
            let end = matched.end();
            let matched_text = &txt[start..end];
            if is_numeric(matched_text) {
                result += matched_text;
            } else {
                result += from_str(matched_text)
            }
            
        }
        if let Some(lastmatch) = pattern_reversed.find(&txt_reversed) {
            let start = lastmatch.start();
            let end = lastmatch.end();
            let lastmatch_text = &txt_reversed[start..end];
            println!("lastmatch : {}", lastmatch.as_str());
            println!("lastmatch_text : {}", lastmatch_text);
            if is_numeric(lastmatch_text) {
                result += lastmatch_text;
            } else {
                let to_string_reversed = reverse_string(lastmatch_text.to_string());
                result += from_str(to_string_reversed.as_str());
            }        
        }
        println!("Result: {}", result);
        let to_add: Result<i32, _> = result.parse();
        match to_add {
            Ok(number) => {
                count += number;
            }
            Err(err) => {
                println!("Error converting to number: {}", err);
            }
        }
    }
    println!("Count: {}", count);
    Ok(())
}


