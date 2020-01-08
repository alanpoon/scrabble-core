#[derive(Debug, Clone)]
struct ScrabbleTile(char); // Should be a-z or ' '

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
    pub fn char(&self) -> char {
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
pub struct ScrabbleBoard {
    placed: Vec<Vec<Option<ScrabbleTile>>>,
    modifiers: Vec<Vec<ScoreModifier>>,
}

impl ScrabbleBoard {
    pub fn display(&self) -> String {
        let mut result = String::with_capacity((self.placed.len() + 1) ^ 2);
        for (tile_row, mod_row) in self.placed.iter().zip(self.modifiers.iter()) {
            for (tile, modifier) in tile_row.iter().zip(mod_row.iter()) {
                let next_char = match tile {
                    Some(tile) => tile.0,
                    None => modifier.char(),
                };
                result.push(next_char);
            }
            result.push('\n');
        }
        result
    }

    pub fn default() -> ScrabbleBoard {
        const SIDE_LENGTH: usize = 15;
        let placed = vec![vec![None; SIDE_LENGTH]; SIDE_LENGTH];
        let modifiers = placed
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                (0..row.len())
                    .map(|col_idx| ScrabbleBoard::modifier(row_idx as i32, col_idx as i32))
                    .collect()
            })
            .collect();
        ScrabbleBoard { placed, modifiers }
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
    tiles: Vec<ScrabbleTile>,
}

#[derive(Debug, Clone)]
pub struct ScrabbleState {
    board: ScrabbleBoard,
    rack: ScrabbleRack,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
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
