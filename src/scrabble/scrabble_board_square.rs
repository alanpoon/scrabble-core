use super::cross_checks::CrossChecks;

#[derive(Debug, Clone, Copy)]
pub struct ScrabbleBoardSquare {
    pub tile: Option<ScrabbleTile>,
    pub cross_checks: Option<CrossChecks>,
    pub modifier: ScoreModifier,
}

impl ScrabbleBoardSquare {
    pub fn is_occupied(&self) -> bool {
        self.tile.is_some()
    }

    pub fn is_anchor(&self) -> bool {
        self.cross_checks.is_some()
    }
}

impl Default for ScrabbleBoardSquare {
    fn default() -> Self {
        ScrabbleBoardSquare {
            tile: None,
            cross_checks: None,
            modifier: ScoreModifier::Plain,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct ScrabbleTile(pub char); // Should be a-z or ' '

#[derive(Debug, Clone, Copy)]
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
