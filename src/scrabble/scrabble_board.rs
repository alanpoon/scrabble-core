use crate::scrabble::cross_checks::CrossChecks;
use crate::scrabble::scoring::ScoreModifier;
use crate::scrabble::scrabble_board_square::{CheckedBoardSquare, CheckedRowSquare};
use crate::scrabble::Direction;
use crate::trie::{Trie, TrieNode};
use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

pub const BOARD_SIZE: usize = 15;

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

    pub fn aisle(&self, direction: Direction) -> usize {
        match direction {
            Direction::Horizontal => self.row,
            Direction::Vertical => self.col,
        }
    }

    pub fn cross(&self, direction: Direction) -> usize {
        match direction {
            Direction::Horizontal => self.col,
            Direction::Vertical => self.row,
        }
    }

    pub fn set_aisle(&mut self, direction: Direction, value: usize) {
        match direction {
            Direction::Horizontal => self.row = value,
            Direction::Vertical => self.col = value,
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

    pub fn step_multi(&self, direction: Direction, steps: isize) -> Position {
        match direction {
            Direction::Horizontal => Position {
                col: (self.col as isize + steps) as usize,
                ..*self
            },
            Direction::Vertical => Position {
                row: (self.col as isize + steps) as usize,
                ..*self
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScrabbleBoard {
    pub squares: [[Option<char>; BOARD_SIZE]; BOARD_SIZE],
}

impl ScrabbleBoard {
    pub fn add_word(&mut self, word: &str, start: Position, direction: Direction) {
        let mut start = start;
        for ch in word.chars() {
            self[start] = Some(ch);
            start = start.step(direction);
        }
    }

    pub fn display(&self) -> String {
        let mut result = String::with_capacity(BOARD_SIZE * (BOARD_SIZE + 1));
        for (row, row_contents) in self.squares.iter().enumerate() {
            for (col, square) in row_contents.iter().enumerate() {
                let position = Position { row, col };
                let next_char = match &square {
                    Some(ch) => *ch,
                    None => ScoreModifier::at(position).as_char(),
                };
                result.push(next_char);
            }
            result.push('\n');
        }
        result
    }

    pub fn to_checked_board(&self, trie: &Trie) -> CheckedScrabbleBoard {
        let mut checked_board = CheckedScrabbleBoard::default();
        for direction in Direction::iterator() {
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    let position = Position { row, col };
                    let square = &mut checked_board[position];
                    let tile = self[position];
                    if tile.is_some() {
                        square.tile = tile;
                    } else {
                        let preceding = self.preceding(position, *direction);
                        let following = self.following(position, *direction);
                        if preceding.is_some() || following.is_some() {
                            let preceding = CrossChecks::unwrap_or_empty(preceding.as_ref());
                            let following = CrossChecks::unwrap_or_empty(following.as_ref());
                            *square.checks_mut(*direction) =
                                Some(CrossChecks::create(trie, preceding, following));
                        }
                    }
                }
            }
        }
        checked_board
    }

    fn preceding(&self, position: Position, direction: Direction) -> Option<String> {
        let mut position = position;
        let mut result: VecDeque<char> = VecDeque::new();
        let cross_idx = position.cross(direction);
        if cross_idx > 0 {
            for preceding_cross_idx in (0..cross_idx).rev() {
                position.set_cross(direction, preceding_cross_idx);
                if let Some(tile) = self[position] {
                    result.push_front(tile);
                } else {
                    break;
                }
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result.iter().collect())
        }
    }

    fn following(&self, position: Position, direction: Direction) -> Option<String> {
        let mut position = position;
        let mut result = String::new();
        let cross_idx = position.cross(direction);
        if cross_idx < BOARD_SIZE {
            for following_cross_idx in cross_idx + 1..BOARD_SIZE {
                position.set_cross(direction, following_cross_idx);
                if let Some(tile) = self[position] {
                    result.push(tile);
                } else {
                    break;
                }
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl Index<Position> for ScrabbleBoard {
    type Output = Option<char>;
    fn index(&self, position: Position) -> &Self::Output {
        &self.squares[position.row][position.col]
    }
}

impl IndexMut<Position> for ScrabbleBoard {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.squares[position.row][position.col]
    }
}

impl Default for ScrabbleBoard {
    fn default() -> ScrabbleBoard {
        ScrabbleBoard {
            squares: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckedScrabbleBoard {
    pub squares: [[CheckedBoardSquare; BOARD_SIZE]; BOARD_SIZE],
}

impl CheckedScrabbleBoard {
    pub fn aisle(&self, direction: Direction, index: usize) -> [CheckedRowSquare; BOARD_SIZE] {
        let mut aisle_contents: [CheckedRowSquare; BOARD_SIZE] = Default::default();
        for cross in 0..BOARD_SIZE {
            let position = Position::from_aisle_cross(direction, index, cross);
            let square = &self[position];
            aisle_contents[cross] = square.to_checked_row_square(direction);
        }
        aisle_contents
    }
}

impl Index<Position> for CheckedScrabbleBoard {
    type Output = CheckedBoardSquare;
    fn index(&self, position: Position) -> &Self::Output {
        &self.squares[position.row][position.col]
    }
}

impl IndexMut<Position> for CheckedScrabbleBoard {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.squares[position.row][position.col]
    }
}

impl Default for CheckedScrabbleBoard {
    fn default() -> CheckedScrabbleBoard {
        let squares: [[CheckedBoardSquare; BOARD_SIZE]; BOARD_SIZE] = Default::default();
        CheckedScrabbleBoard { squares }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_display() {
        let expected = "\
6  2       2  6
 4   3   3   4 
  4   2 2   4  
2  4   2   4  2
    4     4    
 3   3   3   3 
  2   2 2   2  
   2   4   2   
  2   2 2   2  
 3   3   3   3 
    4     4    
2  4   2   4  2
  4   2 2   4  
 4   3   3   4 
6  2       2  6
";
        let actual = ScrabbleBoard::default().display();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_add_word() {
        let expected = "\
6  2       2  6
 4   3   3   4 
  4   2 2   4  
2  4   2   4  2
    4     4    
 3   3   3   3 
  2   2 2   2  
   2   hello   
  2   2 2   2  
 3   3   3   3 
    4     4    
2  4   2   4  2
  4   2 2   4  
 4   3   3   4 
6  2       2  6
";
        let mut board = ScrabbleBoard::default();
        let start = Position { row: 7, col: 7 };
        board.add_word("hello", start, Direction::Horizontal);
        let actual = board.display();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_preceding_horizontal() {
        let mut board = ScrabbleBoard::default();
        let word = "hello";
        let start = Position { row: 7, col: 7 };
        board.add_word(word, start, Direction::Horizontal);

        let expected = word;
        let position = Position { row: 7, col: 12 };
        let actual = board.preceding(position, Direction::Horizontal);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_preceding_vertical() {
        let mut board = ScrabbleBoard::default();
        let word = "hello";
        let start = Position { row: 7, col: 7 };
        board.add_word(word, start, Direction::Vertical);

        let expected = word;
        let position = Position { row: 12, col: 7 };
        let actual = board.preceding(position, Direction::Vertical);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_following_horizontal() {
        let mut board = ScrabbleBoard::default();
        let word = "hello";
        let start = Position { row: 7, col: 7 };
        board.add_word(word, start, Direction::Horizontal);

        let expected = word;
        let position = Position { row: 7, col: 6 };
        let actual = board.following(position, Direction::Horizontal);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_following_vertical() {
        let mut board = ScrabbleBoard::default();
        let word = "hello";
        let start = Position { row: 7, col: 7 };
        board.add_word(word, start, Direction::Vertical);

        let expected = word;
        let position = Position { row: 6, col: 7 };
        let actual = board.following(position, Direction::Vertical);
        assert!(actual.is_some());
        assert_eq!(expected, actual.unwrap());
    }
}
