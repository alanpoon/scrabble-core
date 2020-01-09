use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct ScrabbleTile(pub char); // Should be a-z or ' '

#[derive(Debug, Clone)]
pub struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ScrabbleBoardSquare {
    tile: Option<ScrabbleTile>,
    cross_checks: Option<Vec<char>>,
    modifier: ScoreModifier,
}

impl ScrabbleBoardSquare {
    fn blank() -> ScrabbleBoardSquare {
        ScrabbleBoardSquare {
            tile: None,
            cross_checks: None,
            modifier: ScoreModifier::Plain,
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.tile.is_some()
    }

    pub fn is_anchor(&self) -> bool {
        self.cross_checks.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct ScrabbleBoard {
    pub squares: Vec<Vec<ScrabbleBoardSquare>>,
}

impl ScrabbleBoard {
    pub fn transposed(&self) -> ScrabbleBoard {
        let mut transposed_squares = self.squares.clone();
        for row in 0..self.squares.len() {
            for col in 0..self.squares.len() {
                transposed_squares[col][row] = self.squares[row][col].clone();
            }
        }
        ScrabbleBoard {
            squares: transposed_squares,
        }
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

    pub fn default() -> ScrabbleBoard {
        const SIDE_LENGTH: usize = 15;
        let mut squares = vec![vec![ScrabbleBoardSquare::blank(); SIDE_LENGTH]; SIDE_LENGTH];
        for (row, row_squares) in squares.iter_mut().enumerate() {
            for (col, square) in row_squares.iter_mut().enumerate() {
                square.modifier = ScrabbleBoard::modifier(row as i32, col as i32);
            }
        }
        ScrabbleBoard { squares }
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

#[derive(Debug, Clone)]
pub struct ScrabbleRack {
    pub tiles: HashMap<ScrabbleTile, usize>,
}

impl ScrabbleRack {
    pub fn take_tile(&mut self, tile: ScrabbleTile) -> Result<(), ()> {
        match self.tiles.entry(tile).or_insert(0) {
            count if *count > 0 => {
                *count -= 1;
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub fn replace_tile(&mut self, tile: ScrabbleTile) {
        *self.tiles.entry(tile).or_insert(0) += 1;
    }
}

#[derive(Debug, Clone)]
pub struct ScrabbleState {
    pub board: ScrabbleBoard,
    pub rack: ScrabbleRack,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct ScrabblePlay {
    start_position: (usize, usize),
    direction: Direction,
    word: String,
}

#[cfg(test)]
mod test {
    use crate::scrabble::ScrabbleBoard;

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
