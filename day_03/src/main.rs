use std::fs::File;
use std::io::{self, BufRead};

use grid::Grid;

mod gear;
mod grid;
mod number;
mod row;

// use crate::row::Row;

fn main() -> io::Result<()> {
    // Create Grid object of the game board
    let mut grid: Grid = Grid::new();

    // Open the file in read-only mode
    let file = File::open("input.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = io::BufReader::new(file);

    // Iterate over lines in the file
    let mut row_index: u8 = 0;
    for line_result in reader.lines() {
        // Handle each line or any potential errors
        match line_result {
            Ok(line) => {
                grid.add_row(line, row_index);
                row_index += 1;
            },
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }
    grid.find_adjacents();
    println!("[Part1] Result : {}", grid.clone().get_adjacents_sum());
    grid.find_gears();
    println!("[Part2] Result : {}", grid.clone().get_gears_ratio());

    Ok(())
}
