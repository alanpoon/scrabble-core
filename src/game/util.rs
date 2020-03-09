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
        static DIRECTIONS: [Direction; 1] = [Direction::Horizontal];
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
        Position {
            row: aisle,
            col: cross,
        }
    }

    pub fn cross(&self, direction: Direction) -> usize {
        self.col
    }

    pub fn set_cross(&mut self, direction: Direction, value: usize) {
        self.col = value
    }

    pub fn step(&self, direction: Direction) -> Position {
        Position {
            col: self.col + 1,
            ..*self
        }
    }
}
