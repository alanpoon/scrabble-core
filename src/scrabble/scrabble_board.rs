use crate::scrabble::cross_checks::CrossChecks;
use crate::scrabble::scrabble_board_square::{CheckedBoardSquare, CheckedRowSquare, ScoreModifier};
use crate::scrabble::Direction;
use crate::trie::{Trie, TrieNode};

pub const BOARD_SIZE: usize = 15;

#[derive(Debug, Clone)]
pub struct ScrabbleBoard {
    pub squares: [[Option<char>; BOARD_SIZE]; BOARD_SIZE],
}

impl ScrabbleBoard {
    pub fn to_checked_board(&self, vocab_trie: &Trie) -> CheckedScrabbleBoard {
        let mut checked_board = CheckedScrabbleBoard::default();
        self.add_all_aisle_checks(vocab_trie, &mut checked_board, Direction::Horizontal);
        self.add_all_aisle_checks(vocab_trie, &mut checked_board, Direction::Vertical);
        checked_board
    }

    // TODO: Need to split up this method
    fn add_all_aisle_checks(
        &self,
        vocab_trie: &Trie,
        checked_board: &mut CheckedScrabbleBoard,
        direction: Direction,
    ) {
        let root = vocab_trie.root();

        for aisle_idx in 0..BOARD_SIZE {
            let mut maybe_node: Option<&TrieNode> = None;
            for cross_idx in 0..BOARD_SIZE {
                let (row_idx, col_idx) = match direction {
                    Direction::Horizontal => (aisle_idx, cross_idx),
                    Direction::Vertical => (cross_idx, aisle_idx),
                };
                let ch = self.squares[row_idx][col_idx];
                let checked = &mut checked_board.squares[row_idx][col_idx];

                // Start repeated:
                if let Some(ch) = ch {
                    match maybe_node {
                        Some(node) => maybe_node = node.children.get(&ch),
                        None => maybe_node = root.children.get(&ch),
                    };
                } else if let Some(node) = maybe_node {
                    let checks = match direction {
                        Direction::Horizontal => &mut checked.horizontal_cross_checks,
                        Direction::Vertical => &mut checked.vertical_cross_checks,
                    };
                    if checks.is_none() {
                        *checks = Some(CrossChecks::default());
                    }
                    let checks = checks.as_mut().unwrap();
                    for &ch in node.children.keys() {
                        checks.allow(ch);
                    }
                    maybe_node = None;
                }
                // End repeated
            }

            maybe_node = None;
            for cross_idx in 0..BOARD_SIZE {
                let cross_idx = BOARD_SIZE - 1 - cross_idx; // iterate in reverse
                let (row_idx, col_idx) = match direction {
                    Direction::Horizontal => (aisle_idx, cross_idx),
                    Direction::Vertical => (cross_idx, aisle_idx),
                };

                // Repeat the process, but reversed
                let ch = self.squares[row_idx][col_idx];
                let checked = &mut checked_board.squares[row_idx][col_idx];

                if let Some(ch) = ch {
                    match maybe_node {
                        Some(node) => maybe_node = node.children.get(&ch),
                        None => maybe_node = root.children.get(&ch),
                    };
                } else if let Some(node) = maybe_node {
                    let checks = match direction {
                        Direction::Horizontal => &mut checked.horizontal_cross_checks,
                        Direction::Vertical => &mut checked.vertical_cross_checks,
                    };
                    if checks.is_none() {
                        *checks = Some(CrossChecks::default());
                    }
                    let checks = checks.as_mut().unwrap();
                    for &ch in node.children.keys() {
                        checks.allow(ch);
                    }
                    maybe_node = None;
                }
                // End repeated
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckedScrabbleBoard {
    pub squares: [[CheckedBoardSquare; BOARD_SIZE]; BOARD_SIZE],
}

impl CheckedScrabbleBoard {
    pub fn row(&self, direction: Direction, index: usize) -> [CheckedRowSquare; BOARD_SIZE] {
        let mut row: [CheckedRowSquare; BOARD_SIZE] = Default::default();
        for i in 0..BOARD_SIZE {
            let square = match direction {
                Direction::Horizontal => &self.squares[index][i],
                Direction::Vertical => &self.squares[i][index],
            };
            row[i] = square.to_checked_row_square(direction);
        }
        row
    }
}

impl Default for CheckedScrabbleBoard {
    fn default() -> CheckedScrabbleBoard {
        let squares: [[CheckedBoardSquare; BOARD_SIZE]; BOARD_SIZE] = Default::default();
        CheckedScrabbleBoard { squares }
    }
}

impl ScrabbleBoard {
    pub fn add_word(
        &mut self,
        word: &str,
        start_row: usize,
        start_col: usize,
        direction: Direction,
    ) {
        let (mut row, mut col) = (start_row, start_col);
        for ch in word.chars() {
            self.squares[row][col] = Some(ch);
            match direction {
                Direction::Horizontal => col += 1,
                Direction::Vertical => row += 1,
            }
        }
    }

    pub fn display(&self) -> String {
        let mut result = String::with_capacity(BOARD_SIZE * (BOARD_SIZE + 1));
        for (row_idx, row) in self.squares.iter().enumerate() {
            for (col_idx, square) in row.iter().enumerate() {
                let next_char = match &square {
                    Some(ch) => *ch,
                    None => ScrabbleBoard::modifier(row_idx, col_idx).as_char(),
                };
                result.push(next_char);
            }
            result.push('\n');
        }
        result
    }

    fn modifier(row_idx: usize, col_idx: usize) -> ScoreModifier {
        let (row_idx, col_idx) = (row_idx as i32, col_idx as i32);
        match ((7 - row_idx).abs(), (7 - col_idx).abs()) {
            (x, y) if x == y => match x {
                1 => ScoreModifier::DoubleLetter,
                2 => ScoreModifier::TripleLetter,
                7 => ScoreModifier::TripleWord,
                _ => ScoreModifier::DoubleWord,
            },
            (x, y) if x % 7 == 0 || y % 7 == 0 => match (x + y) % 7 {
                4 => ScoreModifier::DoubleLetter,
                7 => ScoreModifier::TripleWord,
                _ => ScoreModifier::Plain,
            },
            (x, y) if (x - y).abs() == 4 => match (x + y) % 7 {
                1 => ScoreModifier::TripleLetter,
                6 => ScoreModifier::DoubleLetter,
                _ => unreachable!(),
            },
            _ => ScoreModifier::Plain,
        }
    }
}

impl Default for ScrabbleBoard {
    fn default() -> ScrabbleBoard {
        ScrabbleBoard {
            squares: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
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
        board.add_word("hello", 7, 7, Direction::Horizontal);
        let actual = board.display();
        assert_eq!(expected, actual);
    }
}
