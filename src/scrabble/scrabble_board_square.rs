use super::cross_checks::CrossChecks;
use crate::scrabble::Direction;

#[derive(Debug, Clone)]
pub struct CheckedBoardSquare {
    pub tile: Option<char>,
    /// The checks determined by horizontal neighbors (for use while solving a vertical row ):
    pub horizontal_cross_checks: Option<CrossChecks>,
    /// The checks determined by vertical neighbors (for use while solving a horizontal row):
    pub vertical_cross_checks: Option<CrossChecks>,
}

impl CheckedBoardSquare {
    pub fn to_checked_row_square(&self, direction: Direction) -> CheckedRowSquare {
        let cross_checks = match direction {
            Direction::Horizontal => self.horizontal_cross_checks.clone(),
            Direction::Vertical => self.vertical_cross_checks.clone(),
        };
        let is_anchor =
            self.horizontal_cross_checks.is_some() || self.vertical_cross_checks.is_some();
        CheckedRowSquare {
            tile: self.tile,
            cross_checks,
            is_anchor,
        }
    }

    pub fn checks(&self, neighbors: Direction) -> &Option<CrossChecks> {
        match neighbors {
            Direction::Horizontal => &self.horizontal_cross_checks,
            Direction::Vertical => &self.vertical_cross_checks,
        }
    }

    pub fn checks_mut(&mut self, neighbors: Direction) -> &mut Option<CrossChecks> {
        match neighbors {
            Direction::Horizontal => &mut self.horizontal_cross_checks,
            Direction::Vertical => &mut self.vertical_cross_checks,
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
    /// Need to explicitly track whether a square is an anchor since we only have one of the cross checks
    pub is_anchor: bool,
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
            is_anchor: false,
        }
    }
}
