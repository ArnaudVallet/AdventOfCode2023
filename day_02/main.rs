mod classes;

use std::fs::File;
use std::io::{self, BufRead};
use classes::game::Game;

fn main() -> io::Result<()> {

    let red_init = 12;
    let green_init = 13;
    let blue_init = 14;
    let mut result = 0;
    let mut power_result = 0;

    // Open the file in read-only mode
    let file = File::open("input.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    for line_result in reader.lines() {
        // Handle each line or any potential errors
        match line_result {
            Ok(line) => {
                let game = Game::new(&line);
                if game.is_possible(red_init, green_init, blue_init) {
                    result += game.get_id();
                }
                power_result += game.get_set_power();
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
    println!("[Part1] Result : {}", result);
    println!("[Part2] Power Result : {}", power_result);
    Ok(())

}
