use crate::data_structures::Dawg;
use crate::scrabble::{
    letter_value, CheckedRowSquare, CheckedScrabbleBoard, Direction, Position, ScoreModifier,
    ScrabbleBoard, ScrabblePlay, ScrabbleRack, ScrabbleState, BOARD_SIZE,
};

#[derive(Debug)]
pub struct SolvingAisle {
    direction: Direction,
    index: usize,
    squares: [CheckedRowSquare; BOARD_SIZE],
}

impl SolvingAisle {
    pub fn play(&self, start_word_index: usize, word: String) -> ScrabblePlay {
        let start = self.position(start_word_index);
        let score = self.score(start_word_index, &word);
        ScrabblePlay {
            start,
            direction: self.direction,
            word,
            score,
        }
    }

    fn position(&self, cross: usize) -> Position {
        Position::from_aisle_cross(self.direction, self.index, cross)
    }

    fn score(&self, start_word_index: usize, word: &str) -> u32 {
        let mut position = self.position(start_word_index);
        let mut score = 0;
        let mut new_word_score = 0;
        let mut new_word_multiplier = 1;

        for (i, ch) in word.chars().enumerate() {
            let square = &self.squares[start_word_index + i];
            let score_modifier = match square.tile {
                Some(_) => ScoreModifier::Plain,
                None => ScoreModifier::at(position),
            };
            let word_multiplier = score_modifier.word_multiplier();
            let ch_value = score_modifier.letter_multiplier() * letter_value(ch);
            new_word_score += ch_value;
            new_word_multiplier *= word_multiplier;
            if let Some(cross_checks) = &square.cross_checks {
                score += (cross_checks.cross_sum + ch_value) * word_multiplier;
            }
            position = position.step(self.direction);
        }
        score += new_word_score * new_word_multiplier;
        score
    }
}

pub struct SolvingAnchor<'a> {
    solving_row: &'a SolvingAisle,
    anchor_index: usize,
}

impl<'a> SolvingAnchor<'a> {
    pub fn plays(&self, rack: &ScrabbleRack, root: &TrieNode) -> Vec<ScrabblePlay> {
        let mut plays: Vec<ScrabblePlay> = Vec::new();
        let mut rack = rack.clone();
        let mut partial_word = String::with_capacity(BOARD_SIZE);
        self.add_plays_for_left(&mut plays, &mut rack, &mut partial_word, root, self.limit());
        plays
    }

    fn limit(&self) -> usize {
        for limit in 0..self.anchor_index {
            let square = &self.solving_row.squares[self.anchor_index - (limit + 1)];
            if square.tile.is_some() || square.is_anchor {
                return limit;
            }
        }
        self.anchor_index
    }

    fn add_plays_for_left(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        rack: &mut ScrabbleRack,
        partial_word: &mut String,
        node: &TrieNode,
        limit: usize,
    ) {
        let starting_index = {
            let mut starting_index = self.anchor_index;
            while starting_index > 1 && self.solving_row.squares[starting_index - 1].tile.is_some()
            {
                starting_index -= 1;
            }
            starting_index
        };
        //println!(
        //    "Initial-extending {:?} in aisle {} with partial_word \"{}\" starting at cross {}",
        //    self.solving_row.direction, self.solving_row.index, partial_word, starting_index,
        //);
        self.extend_right(plays, rack, partial_word, node, starting_index, true);
        if limit > 0 {
            for (c, subnode) in node.children.iter() {
                if rack.take_tile(*c).is_ok() {
                    partial_word.push(*c);
                    self.add_plays_for_left(plays, rack, partial_word, subnode, limit - 1);
                    partial_word.pop();
                    rack.add_tile(*c);
                }
            }
        }
    }

    fn extend_right(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        rack: &mut ScrabbleRack,
        partial_word: &mut String,
        node: &TrieNode,
        index: usize,
        initial: bool,
    ) {
        if index >= BOARD_SIZE {
            return;
        }
        let square = &self.solving_row.squares[index];
        if !initial {
            match self.current_play(partial_word, node, index, square) {
                Some(play) => plays.push(play),
                _ => {}
            }
        }

        if let Some(ch) = square.tile {
            let next_node = node.children.get(&ch);
            if let Some(subnode) = next_node {
                partial_word.push(ch);
                //println!(
                //    "Pushed {} at {:?} (already on board) to get \"{}\". Attempting to extend to {:?}",
                //    ch,
                //    self.solving_row.position(index),
                //    partial_word,
                //    self.solving_row.position(index + 1),
                //);
                self.extend_right(plays, rack, partial_word, subnode, index + 1, false);
                let popped = partial_word.pop().unwrap();
                //println!(
                //    "Dropped {} from partial_word to go back to \"{}\" (ignoring square on board)",
                //    popped, partial_word
                //);
            }
        } else {
            for (ch, subnode) in node.children.iter() {
                if !rack.take_tile(*ch).is_ok() {
                    continue;
                }
                let allowed = match &square.cross_checks {
                    Some(cross_checks) => cross_checks.is_allowed(*ch),
                    None => true,
                };
                if !allowed {
                    continue;
                }
                partial_word.push(*ch);
                //println!(
                //    "Pushed {} at {:?} (from rack) to get \"{}\". Attempting to extend to {:?}",
                //    ch,
                //    self.solving_row.position(index),
                //    partial_word,
                //    self.solving_row.position(index + 1),
                //);
                self.extend_right(plays, rack, partial_word, subnode, index + 1, false);
                partial_word.pop();
                rack.add_tile(*ch);
                //println!(
                //    "Dropped {} from partial_word to go back to \"{}\" (restored to rack)",
                //    ch, partial_word
                //);
            }
        }
    }

    fn current_play(
        &self,
        partial_word: &str,
        node: &TrieNode,
        index: usize,
        square: &CheckedRowSquare,
    ) -> Option<ScrabblePlay> {
        if let Some(c) = square.tile {
            if index < BOARD_SIZE - 1 {
                return None;
            }
            if let Some(child) = node.children.get(&c) {
                if child.terminal {
                    let start = index - partial_word.len() + 1;
                    //println!(
                    //    "Found a play! {} starting at {:?} (1)",
                    //    partial_word,
                    //    self.solving_row.position(start)
                    //);
                    let play = self.solving_row.play(start, partial_word.to_string());
                    return Some(play);
                }
            }
            None
        } else {
            if node.terminal {
                let start = index - partial_word.len();
                //println!("Found a play! {} starting at {} (2)", partial_word, start);
                Some(self.solving_row.play(start, partial_word.to_string()))
            } else {
                None
            }
        }
    }
}

pub struct PlayGenerator {
    pub dawg: Dawg,
    pub checked_board: CheckedScrabbleBoard,
    pub rack: ScrabbleRack,
}

impl PlayGenerator {
    pub fn plays(&self) -> Vec<ScrabblePlay> {
        let mut plays: Vec<ScrabblePlay> = Vec::new();
        for solving_row in self.solving_aisles() {
            for (anchor_index, tile) in solving_row.squares.iter().enumerate() {
                if !tile.is_anchor {
                    continue;
                }
                let solving_anchor = SolvingAnchor {
                    solving_row: &solving_row,
                    anchor_index,
                };
                plays.extend(solving_anchor.plays(&self.state.rack, &self.dawg.root()));
            }
        }
        plays
    }

    fn solving_aisles(&self) -> Vec<SolvingAisle> {
        let board = &self.state.checked_board;
        let mut solving_rows: Vec<SolvingAisle> = Vec::with_capacity(2 * BOARD_SIZE);
        for index in 0..BOARD_SIZE {
            for &direction in Direction::iterator() {
                let solving_row = SolvingAisle {
                    direction,
                    index,
                    squares: board.aisle(direction, index),
                };
                solving_rows.push(solving_row);
            }
        }
        solving_rows
    }
}
