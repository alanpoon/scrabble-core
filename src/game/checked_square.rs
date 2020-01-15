use crate::game::cross_checks::CrossChecks;
use crate::game::util::Direction;

#[derive(Debug, Clone)]
pub struct CheckedBoardSquare {
    pub tile: Option<char>,
    /// The checks determined by horizontal neighbors (for use while solving a vertical row ):
    pub horizontal_cross_checks: Option<CrossChecks>,
    /// The checks determined by vertical neighbors (for use while solving a horizontal row):
    pub vertical_cross_checks: Option<CrossChecks>,
}

impl CheckedBoardSquare {
    pub fn to_checked_row_square(&self, direction: Direction) -> CheckedAisleSquare {
        let cross_checks = match direction {
            Direction::Horizontal => self.vertical_cross_checks.clone(),
            Direction::Vertical => self.horizontal_cross_checks.clone(),
        };
        let is_anchor =
            self.horizontal_cross_checks.is_some() || self.vertical_cross_checks.is_some();
        CheckedAisleSquare {
            tile: self.tile,
            cross_checks,
            is_anchor,
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
pub struct CheckedAisleSquare {
    pub tile: Option<char>,
    pub cross_checks: Option<CrossChecks>,
    /// Need to explicitly track whether a square is an anchor since we only have one of the cross checks
    pub is_anchor: bool,
}

impl CheckedAisleSquare {
    pub fn is_compatible(&self, letter: char) -> bool {
        self.cross_checks
            .as_ref()
            .and_then(|checks| Some(checks.allows(letter)))
            .unwrap_or(true)
    }
}

impl Default for CheckedAisleSquare {
    fn default() -> CheckedAisleSquare {
        CheckedAisleSquare {
            tile: None,
            cross_checks: None,
            is_anchor: false,
        }
    }
}
