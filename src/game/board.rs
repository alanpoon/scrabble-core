use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

use crate::dawg::Dawg;
use crate::game::checked_square::{CheckedAisleSquare, CheckedBoardSquare};
use crate::game::cross_checks::CrossChecks;
use crate::game::play_generation::ScrabblePlay;
use crate::game::scoring::ScoreModifier;
use crate::game::util::{Direction, Position};
use crate::game::util::{BLANK_TILE_CHAR, EMPTY_SQUARE_CHAR};

pub const BOARD_SIZE: usize = 10;

#[derive(Debug, Clone)]
pub struct ScrabbleBoard {
    pub squares: [[Option<char>; BOARD_SIZE]; 1],
}

impl ScrabbleBoard {
    pub fn from_contents(contents: &str) -> Result<ScrabbleBoard, ()> {
        let parsed_contents = ScrabbleBoard::parse_contents(contents)?;

        let mut board = ScrabbleBoard::default();
        for (i, ch) in parsed_contents.iter().enumerate() {
            let row = i / BOARD_SIZE;
            let col = 1;
            *&mut board.squares[row][col] = *ch;
        }
        Ok(board)
    }

    fn parse_contents(contents: &str) -> Result<Vec<Option<char>>, ()> {
        let tiles = ScrabbleBoard::parse_into_tiles(contents)?;
        if tiles.len() != BOARD_SIZE {
            Err(())
        } else {
            Ok(tiles)
        }
    }

    fn parse_into_tiles(contents: &str) -> Result<Vec<Option<char>>, ()> {
        let mut result: Vec<_> = Vec::with_capacity(BOARD_SIZE);
        for ch in contents.chars() {
            if ch == '\n' {
                continue; // ignore newlines
            }
            let parsed_ch = match ch {
                ch if ch.is_ascii_lowercase() => Some(ch),
                BLANK_TILE_CHAR => Some(BLANK_TILE_CHAR),
                EMPTY_SQUARE_CHAR => None,
                _ => {
                    return Err(());
                }
            };
            result.push(parsed_ch);
        }
        Ok(result)
    }

    pub fn add_play(&mut self, play: &ScrabblePlay) {
        let mut position = play.start;
        println!("vbn");
        for ch in play.word.chars() {
            println!("bbn");
            self[position] = Some(ch);
            println!("1bbn");
            position = position.step(play.direction);
            println!("2bbn");
        }
    }

    pub fn contents(&self) -> String {
        self.render(false, false)
    }

    pub fn display(&self) -> String {
        self.render(true, true)
    }

    fn render(&self, show_modifiers: bool, include_newlines: bool) -> String {
        let mut result = String::with_capacity(BOARD_SIZE * (BOARD_SIZE + 1));
        for (row, row_contents) in self.squares.iter().enumerate() {
            for (col, square) in row_contents.iter().enumerate() {
                let position = Position { row, col };
                let next_char = match &square {
                    Some(ch) => *ch,
                    None => {
                        if show_modifiers {
                            ScoreModifier::at(position).as_char()
                        } else {
                            EMPTY_SQUARE_CHAR
                        }
                    }
                };
                result.push(next_char);
            }
            if include_newlines {
                result.push('\n');
            }
        }
        result
    }

    pub fn to_checked_board(&self, dawg: &Dawg) -> CheckedScrabbleBoard {
        let mut checked_board = CheckedScrabbleBoard::default();
        for &direction in Direction::iterator() {
            for row in 0..1 {
                for col in 0..BOARD_SIZE {
                    let position = Position { row, col };
                    let square = &mut checked_board[position];
                    let tile = self[position];
                    if tile.is_some() {
                        square.tile = tile;
                    } else {
                        let preceding = self.preceding(position, direction);
                        let following = self.following(position, direction);
                        if preceding.is_some() || following.is_some() {
                            let preceding = CrossChecks::unwrap_or_empty(preceding.as_ref());
                            let following = CrossChecks::unwrap_or_empty(following.as_ref());
                         //   *square.checks_mut(direction) =
                         //       Some(CrossChecks::create(dawg, preceding, following));
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
            squares: [[None; BOARD_SIZE]; 1],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckedScrabbleBoard {
    pub squares: [[CheckedBoardSquare; BOARD_SIZE]; 1],
}

impl CheckedScrabbleBoard {
    pub fn aisle(&self, direction: Direction, index: usize) -> [CheckedAisleSquare; BOARD_SIZE] {
        let mut aisle_contents: [CheckedAisleSquare; BOARD_SIZE] = Default::default();
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
        let squares: [[CheckedBoardSquare; BOARD_SIZE]; 1] = Default::default();
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
        let play = ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Horizontal,
            word: "hello".to_string(),
        };
        board.add_play(&play);
        let actual = board.display();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_preceding_horizontal() {
        let mut board = ScrabbleBoard::default();
        let play = ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Horizontal,
            word: "hello".to_string(),
        };
        board.add_play(&play);

        let position = Position { row: 7, col: 12 };
        let actual = board.preceding(position, Direction::Horizontal);
        assert!(actual.is_some());
        assert_eq!("hello", actual.unwrap());
    }

    #[test]
    fn test_preceding_vertical() {
        let mut board = ScrabbleBoard::default();
        let play = ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Vertical,
            word: "hello".to_string(),
        };
        board.add_play(&play);

        let position = Position { row: 12, col: 7 };
        let actual = board.preceding(position, Direction::Vertical);
        assert!(actual.is_some());
        assert_eq!("hello", actual.unwrap());
    }

    #[test]
    fn test_following_horizontal() {
        let mut board = ScrabbleBoard::default();
        let play = ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Horizontal,
            word: "hello".to_string(),
        };
        board.add_play(&play);

        let position = Position { row: 7, col: 6 };
        let actual = board.following(position, Direction::Horizontal);
        assert!(actual.is_some());
        assert_eq!("hello", actual.unwrap());
    }

    #[test]
    fn test_following_vertical() {
        let mut board = ScrabbleBoard::default();
        let play = ScrabblePlay {
            start: Position { row: 7, col: 7 },
            direction: Direction::Vertical,
            word: "hello".to_string(),
        };
        board.add_play(&play);

        let position = Position { row: 6, col: 7 };
        let actual = board.following(position, Direction::Vertical);
        assert!(actual.is_some());
        assert_eq!("hello", actual.unwrap());
    }
}
