use crate::data_structures::{Dawg, DawgEdge, DawgNodeIndex};
use crate::scrabble::{
    letter_value, CheckedRowSquare, CheckedScrabbleBoard, Direction, Position, ScoreModifier,
    ScrabblePlay, ScrabbleRack, BOARD_SIZE,
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
        let mut tiles_used: u8 = 0;

        for (i, ch) in word.chars().enumerate() {
            let square = &self.squares[start_word_index + i];
            let score_modifier = match square.tile {
                Some(_) => ScoreModifier::Plain,
                None => {
                    tiles_used += 1;
                    ScoreModifier::at(position)
                }
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
        if tiles_used == 7 {
            score += 50;
        }
        score
    }
}

pub struct SolvingAnchor<'a> {
    solving_row: &'a SolvingAisle,
    anchor_index: usize,
    dawg: &'a Dawg,
}

impl<'a> SolvingAnchor<'a> {
    pub fn plays(&self, rack: &ScrabbleRack) -> Vec<ScrabblePlay> {
        // TODO: Put partial word, node, and edge-to-node into a single struct that gets passed around?
        let mut plays: Vec<ScrabblePlay> = Vec::new();
        let mut rack = rack.clone();
        let partial_word_node = self.initial_partial_word_node();
        let mut partial_word = partial_word_node.0;

        let initial_limit = self.initial_limit();

        let node = partial_word_node.1;
        if node.is_some() {
            self.add_plays_for_left(
                &mut plays,
                &mut rack,
                &mut partial_word,
                node.unwrap(),
                initial_limit,
            );
        }
        plays
    }

    fn initial_partial_word_node(&self) -> (String, Option<DawgNodeIndex>) {
        let first_filled_index = {
            let mut index: Option<usize> = None;
            for possible_first in (0..self.anchor_index).rev() {
                if self.solving_row.squares[possible_first].tile.is_none() {
                    index = Some(possible_first + 1);
                    break;
                }
            }
            index.unwrap_or(0)
        };

        let mut partial_word = String::with_capacity(BOARD_SIZE);
        let mut node = Some(self.dawg.root());
        for index in first_filled_index..self.anchor_index {
            let ch = self.solving_row.squares[index].tile.unwrap();
            partial_word.push(ch);
            node = node.and_then(|i| self.dawg.leaving_edge(i, ch).and_then(|edge| edge.target))
        }
        (partial_word, node)
    }

    fn initial_limit(&self) -> usize {
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
        node: DawgNodeIndex,
        limit: usize,
    ) {
        self.extend_right(plays, rack, partial_word, node, self.anchor_index);
        if limit > 0 {
            self.dawg.apply_to_child_edges(node, |edge| {
                if edge.target.is_none() {
                    return;
                }
                let ch = edge.letter;
                let tile = rack.take_tile(ch);
                if tile.is_err() {
                    return;
                }
                partial_word.push(ch);
                self.add_plays_for_left(plays, rack, partial_word, edge.target.unwrap(), limit - 1);
                partial_word.pop();
                rack.add_tile(tile.unwrap());
            });
        }
    }

    fn extend_right(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        rack: &mut ScrabbleRack,
        partial_word: &mut String,
        node: DawgNodeIndex,
        next_tile_index: usize,
    ) {
        if next_tile_index >= BOARD_SIZE {
            return;
        }
        let next_square = &self.solving_row.squares[next_tile_index];
        if let Some(ch) = next_square.tile {
            if let Some(edge) = self.dawg.leaving_edge(node, ch) {
                self.extend_using_edge(plays, rack, partial_word, next_tile_index, edge);
            }
        } else {
            self.dawg.apply_to_child_edges(node, |edge| {
                let ch = edge.letter;
                let tile = rack.take_tile(ch);
                if tile.is_err() {
                    return;
                }
                let compatible = next_square
                    .cross_checks
                    .as_ref()
                    .and_then(|checks| Some(checks.allows(ch)))
                    .unwrap_or(true);
                if compatible {
                    self.extend_using_edge(plays, rack, partial_word, next_tile_index, edge);
                }
                rack.add_tile(tile.unwrap());
            })
        }
    }

    fn extend_using_edge(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        rack: &mut ScrabbleRack,
        partial_word: &mut String,
        placement_index: usize,
        edge: &DawgEdge,
    ) {
        partial_word.push(edge.letter);
        self.check_add_play(plays, partial_word, edge, placement_index + 1);
        if let Some(target) = edge.target {
            self.extend_right(plays, rack, partial_word, target, placement_index + 1);
        }
        partial_word.pop();
    }

    fn check_add_play(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        partial_word: &str,
        edge: &DawgEdge,
        next_square_index: usize,
    ) {
        if next_square_index < BOARD_SIZE
            && self.solving_row.squares[next_square_index].tile.is_some()
        {
            return; // there is a tile in the next square, so this partial_word isn't a valid play
        }
        if next_square_index < self.anchor_index + 1 {
            return; // Haven't placed anything in the anchor index
        }

        if edge.word_terminator {
            let start = next_square_index - partial_word.len();
            let play = self.solving_row.play(start, partial_word.to_string());
            plays.push(play)
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
                    dawg: &self.dawg,
                    solving_row: &solving_row,
                    anchor_index,
                };
                plays.extend(solving_anchor.plays(&self.rack));
            }
        }
        plays
    }

    fn solving_aisles(&self) -> Vec<SolvingAisle> {
        let board = &self.checked_board;
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
