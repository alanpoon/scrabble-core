use std::time::Instant;

use scrabble::load_dawg;
use scrabble::{board_from_plays,best_card_to_buy, generate_plays, Direction, Position, ScrabblePlay};

fn main() {
    let start = Instant::now();
    load_dawg();
    let duration = start.elapsed();
    println!("Time elapsed in load_dawg() is: {:?}", duration);

    let existing_plays: Vec<ScrabblePlay> = vec![];
    let board = board_from_plays(&existing_plays);
    let start = Instant::now();
    //let plays = generate_plays(vec![143, 135, 108, 110, 111], &board);
    println!("...{:?}",[143, 135, 108, 110, 111]);
    //println!("Time elapsed in generate_plays() is: {:?}", plays);draft: v,q,f,c,n offer_row:d,o,u,s,r
    let buys = best_card_to_buy(vec![107,100,123,142,170],vec![143,135,108,110,111],&board);
    println!("Time elapsed in buys() is: {:?}", buys);
}
