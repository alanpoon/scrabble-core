use std::collections::{HashMap, HashSet};

mod cross_checks;
mod scrabble_board;
mod scrabble_board_square;
mod scrabble_rack;

use crate::scrabble::scrabble_board::ScrabbleBoard;
use crate::scrabble::scrabble_board_square::ScrabbleTile;
use crate::scrabble::scrabble_rack::ScrabbleRack;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct ScrabblePlay {
    start_position: (usize, usize),
    direction: Direction,
    word: String,
}

#[derive(Debug, Clone)]
pub struct ScrabbleState {
    pub board: ScrabbleBoard,
    pub rack: ScrabbleRack,
}
