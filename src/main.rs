use std::time::Instant;

use scrabble::load_dawg;
use scrabble::{board_from_plays, generate_plays, Direction, Position, ScrabblePlay};

fn main() {
    let start = Instant::now();
    load_dawg();
    let duration = start.elapsed();
    println!("Time elapsed in load_dawg() is: {:?}", duration);

    let existing_plays: Vec<ScrabblePlay> = vec![];
    let board = board_from_plays(&existing_plays);
    println!("{}", board.display());

    let start = Instant::now();
    let plays = generate_plays("abcdefg", &board, 1);
    let duration = start.elapsed();
    for play in plays.iter() {
        println!("play {:?}", play);
    }
    println!("Time elapsed in generate_plays() is: {:?}", duration);
}
