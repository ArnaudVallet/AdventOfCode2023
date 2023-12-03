mod classes;

use classes::game::Game;
use classes::set::Set;

fn main() {
    let line =
        "Game 1: 1 red, 5 blue, 1 green; 16 blue, 3 red; 6 blue, 5 red; 4 red, 7 blue, 1 green";
    let game = Game::new(line);
    println!("{:#?}", game);
}
