use std::slice::Iter;

pub const BLANK_TILE_CHAR: char = '_';
pub const EMPTY_SQUARE_CHAR: char = ' ';

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn from_aisle_cross(direction: Direction, aisle: usize, cross: usize) -> Position {
        match direction {
            Direction::Horizontal => Position {
                row: aisle,
                col: cross,
            },
            Direction::Vertical => Position {
                row: cross,
                col: aisle,
            },
        }
    }

    pub fn cross(&self, direction: Direction) -> usize {
        match direction {
            Direction::Horizontal => self.col,
            Direction::Vertical => self.row,
        }
    }

    pub fn set_cross(&mut self, direction: Direction, value: usize) {
        match direction {
            Direction::Horizontal => self.col = value,
            Direction::Vertical => self.row = value,
        }
    }

    pub fn step(&self, direction: Direction) -> Position {
        match direction {
            Direction::Horizontal => Position {
                col: self.col + 1,
                ..*self
            },
            Direction::Vertical => Position {
                row: self.row + 1,
                ..*self
            },
        }
    }
}
