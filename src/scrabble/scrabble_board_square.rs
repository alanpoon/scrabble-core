use super::cross_checks::CrossChecks;
use crate::scrabble::Direction;

#[derive(Debug, Clone)]
pub struct CheckedBoardSquare {
    pub tile: Option<char>,
    /// The checks to use when working with a horizontal solving row (determined by vertical neighbors):
    pub horizontal_cross_checks: Option<CrossChecks>,
    /// The checks to use when working with a vertical solving row (determined by horizontal neighbors):
    pub vertical_cross_checks: Option<CrossChecks>,
}

impl CheckedBoardSquare {
    pub fn to_checked_row_square(&self, direction: Direction) -> CheckedRowSquare {
        let cross_checks = match direction {
            Direction::Horizontal => self.horizontal_cross_checks.clone(),
            Direction::Vertical => self.vertical_cross_checks.clone(),
        };
        CheckedRowSquare {
            tile: self.tile,
            cross_checks,
        }
    }
}

impl Default for CheckedBoardSquare {
    fn default() -> CheckedBoardSquare {
        CheckedBoardSquare {
            tile: None,
            horizontal_cross_checks: None,
            vertical_cross_checks: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckedRowSquare {
    pub tile: Option<char>,
    pub cross_checks: Option<CrossChecks>,
}

impl CheckedRowSquare {
    pub fn is_anchor(&self) -> bool {
        self.cross_checks.is_some()
    }
}

impl Default for CheckedRowSquare {
    fn default() -> CheckedRowSquare {
        CheckedRowSquare {
            tile: None,
            cross_checks: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScoreModifier {
    Plain,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

impl ScoreModifier {
    pub fn as_char(&self) -> char {
        match self {
            ScoreModifier::Plain => ' ',
            ScoreModifier::DoubleLetter => '2',
            ScoreModifier::TripleLetter => '3',
            ScoreModifier::DoubleWord => '4',
            ScoreModifier::TripleWord => '6',
        }
    }
}
