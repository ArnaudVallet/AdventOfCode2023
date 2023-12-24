mod types;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use types::network::Network;


fn main() -> io::Result<()> {

    // State
    let mut network = Network {
        instructions: vec![],
        nodes: HashMap::new()
    };

    // Open the file named input.txt
    let file = File::open("input.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);

    // Iterate over the first two lines in the file
    for line in reader.lines() {
        let line = line?;        
        if line.contains("=") {
            network.add_node(line);
        } else if line.contains("R") {
            network.instructions = line.chars().collect();
        }
    }

    //println!("Result 1: {:#?}", network.steps(String::from("AAA"), String::from("ZZZ")));
    println!("Result 2: {:#?}", network.ghost_steps());


    Ok(())
}
