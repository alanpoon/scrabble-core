use std::time::Instant;

use crate::loading::load_dawg;
use crate::scrabble::{Direction, Position, ScrabbleBoard, ScrabbleRack};
use crate::solving::PlayGenerator;
//use crate::solving::PlayGenerator;

mod data_structures;
mod loading;
mod scrabble;
mod solving;

fn main() {
    let start = Instant::now();
    let dawg = load_dawg();
    let duration = start.elapsed();
    println!("Time elapsed in load_dawg() is: {:?}", duration);

    let start = Instant::now();
    assert!(dawg.contains("zymosimeter"));
    assert!(!dawg.contains("zymosometer"));
    let duration = start.elapsed();
    println!("Time elapsed in vocab_trie.contains() is: {:?}", duration);

    let mut rack = ScrabbleRack::new();
    rack.add_tiles("abcdef ");
    let mut board = ScrabbleBoard::default();
    board.add_word("hello", Position { row: 7, col: 7 }, Direction::Horizontal);
    println!("{}", board.display());

    let start = Instant::now();
    let checked_board = board.to_checked_board(&dawg);
    let generator = PlayGenerator {
        dawg,
        checked_board,
        rack: rack.clone(),
    };
    let mut plays = generator.plays();
    plays.sort_by_key(|x| x.score);

    let duration = start.elapsed();
    for (play, _) in plays.iter().rev().zip(0..20) {
        println!("{:?}", play);
    }

    println!("Time elapsed in generate_plays() is: {:?}", duration);
}
