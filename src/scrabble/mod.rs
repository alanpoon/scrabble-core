mod cross_checks;
mod scrabble_board;
mod scrabble_board_square;
mod scrabble_rack;

use crate::scrabble::scrabble_board::CheckedScrabbleBoard;
pub use crate::scrabble::scrabble_board::{ScrabbleBoard, BOARD_SIZE};
pub use crate::scrabble::scrabble_board_square::{CheckedBoardSquare, CheckedRowSquare};
pub use crate::scrabble::scrabble_rack::ScrabbleRack;
use std::slice::Iter;
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct ScrabblePlay {
    pub start_position: (usize, usize),
    pub direction: Direction,
    pub word: String,
}

#[derive(Debug, Clone)]
pub struct ScrabbleState {
    pub checked_board: CheckedScrabbleBoard,
    pub rack: ScrabbleRack,
}
