use crate::scrabble::scrabble_board_square::{ScoreModifier, ScrabbleBoardSquare};

pub const BOARD_SIZE: usize = 15;

#[derive(Debug, Clone)]
pub struct ScrabbleBoard {
    pub squares: [[ScrabbleBoardSquare; BOARD_SIZE]; BOARD_SIZE],
}

impl ScrabbleBoard {
    pub fn horizontal_row(&self, index: usize) -> [ScrabbleBoardSquare; BOARD_SIZE] {
        self.squares[index]
    }

    pub fn vertical_row(&self, index: usize) -> [ScrabbleBoardSquare; BOARD_SIZE] {
        let mut squares: [ScrabbleBoardSquare; BOARD_SIZE] = Default::default();
        for i in 0..BOARD_SIZE {
            squares[i] = self.squares[i][index]
        }
        squares
    }

    pub fn display(&self) -> String {
        let mut result = String::with_capacity((self.squares.len() + 1) ^ 2);
        for row in self.squares.iter() {
            for square in row.iter() {
                let next_char = match &square.tile {
                    Some(tile) => tile.0,
                    None => square.modifier.as_char(),
                };
                result.push(next_char);
            }
            result.push('\n');
        }
        result
    }

    fn modifier(row_idx: i32, col_idx: i32) -> ScoreModifier {
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
        let mut squares: [[ScrabbleBoardSquare; BOARD_SIZE]; BOARD_SIZE] = Default::default();
        for (row, row_squares) in squares.iter_mut().enumerate() {
            for (col, square) in row_squares.iter_mut().enumerate() {
                square.modifier = ScrabbleBoard::modifier(row as i32, col as i32);
            }
        }
        ScrabbleBoard { squares }
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
}
