use std::slice::Iter;

pub use crate::scrabble::scoring::{letter_value, ScoreModifier};
use crate::scrabble::scrabble_board::CheckedScrabbleBoard;
pub use crate::scrabble::scrabble_board::{Position, ScrabbleBoard, BOARD_SIZE};
pub use crate::scrabble::scrabble_board_square::{CheckedBoardSquare, CheckedRowSquare};
pub use crate::scrabble::scrabble_rack::ScrabbleRack;

mod cross_checks;
mod scoring;
mod scrabble_board;
mod scrabble_board_square;
mod scrabble_rack;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 2] = [Direction::Horizontal, Direction::Vertical];
        DIRECTIONS.iter()
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct ScrabblePlay {
    pub start: Position,
    pub direction: Direction,
    pub word: String,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct ScrabbleState {
    pub checked_board: CheckedScrabbleBoard,
    pub rack: ScrabbleRack,
}
