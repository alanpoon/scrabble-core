use crate::scrabble::scrabble_board::Position;

pub fn letter_value(letter: char) -> u32 {
    match letter {
        ' ' => 0,
        'a' => 1,
        'b' => 3,
        'c' => 3,
        'd' => 2,
        'e' => 1,
        'f' => 4,
        'g' => 2,
        'h' => 4,
        'i' => 1,
        'j' => 8,
        'k' => 5,
        'l' => 1,
        'm' => 3,
        'n' => 1,
        'o' => 1,
        'p' => 3,
        'q' => 10,
        'r' => 1,
        's' => 1,
        't' => 1,
        'u' => 1,
        'v' => 4,
        'w' => 4,
        'x' => 8,
        'y' => 4,
        'z' => 10,
        _ => panic!("Unexpected letter to score: {}", letter),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScoreModifier {
    Plain,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

impl ScoreModifier {
    pub fn word_multiplier(&self) -> u32 {
        match self {
            ScoreModifier::DoubleWord => 2,
            ScoreModifier::TripleWord => 3,
            _ => 1,
        }
    }

    pub fn letter_multiplier(&self) -> u32 {
        match self {
            ScoreModifier::DoubleLetter => 2,
            ScoreModifier::TripleLetter => 3,
            _ => 1,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            ScoreModifier::Plain => ' ',
            ScoreModifier::DoubleLetter => '2',
            ScoreModifier::TripleLetter => '3',
            ScoreModifier::DoubleWord => '4',
            ScoreModifier::TripleWord => '6',
        }
    }

    pub fn at(position: Position) -> ScoreModifier {
        let (row_idx, col_idx) = (position.row as i32, position.col as i32);
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
