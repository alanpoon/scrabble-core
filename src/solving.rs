use crate::data_structures::{Dawg, DawgEdge, DawgNodeIndex, DAWG_EDGE_TO_ROOT};
use crate::scrabble::{
    letter_value, CheckedRowSquare, CheckedScrabbleBoard, Direction, Position, ScoreModifier,
    ScrabbleBoard, ScrabblePlay, ScrabbleRack, BOARD_SIZE,
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
    dawg: &'a Dawg,
}

impl<'a> SolvingAnchor<'a> {
    pub fn plays(&self, rack: &ScrabbleRack) -> Vec<ScrabblePlay> {
        let mut plays: Vec<ScrabblePlay> = Vec::new();
        let mut rack = rack.clone();
        let mut partial_word = self.initial_partial_word();
        let initial_limit = self.initial_limit();

        //        println!(
        //            "SolvingAnchor.plays(): {:?} in aisle {} with anchor index {}, initial limit {}, and partial_word \"{}\" ",
        //            self.solving_row.direction, self.solving_row.index, self.anchor_index, initial_limit, partial_word,
        //        );

        self.add_plays_for_left(
            &mut plays,
            &mut rack,
            &mut partial_word,
            DAWG_EDGE_TO_ROOT,
            self.dawg.root(),
            initial_limit,
        );
        plays
    }

    fn initial_partial_word(&self) -> String {
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
        for index in first_filled_index..self.anchor_index {
            partial_word.push(self.solving_row.squares[index].tile.unwrap())
        }
        partial_word
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
        edge_to_node: &DawgEdge,
        node: DawgNodeIndex,
        limit: usize,
    ) {
        //        println!(
        //            "add_plays_for_left: {:?} in aisle {} with anchor index {} and partial_word \"{}\" ",
        //            self.solving_row.direction, self.solving_row.index, self.anchor_index, partial_word,
        //        );
        self.extend_right(
            plays,
            rack,
            partial_word,
            edge_to_node,
            node,
            self.anchor_index,
        );
        if limit > 0 {
            self.dawg.apply_to_child_edges(node, |edge| {
                if edge.target.is_none() {
                    return;
                }
                let ch = edge.letter;
                if rack.take_tile(ch).is_err() {
                    return;
                }
                partial_word.push(ch);
                self.add_plays_for_left(
                    plays,
                    rack,
                    partial_word,
                    edge,
                    edge.target.unwrap(),
                    limit - 1,
                );
                partial_word.pop();
                rack.add_tile(ch);
            });
        }
    }

    fn extend_right(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        rack: &mut ScrabbleRack,
        partial_word: &mut String,
        edge_to_node: &DawgEdge,
        node: DawgNodeIndex,
        next_tile_index: usize,
    ) {
        //        println!(
        //            "extend_right: {:?} in aisle {} with partial_word \"{}\" and next_tile_index {} ",
        //            self.solving_row.direction, self.solving_row.index, partial_word, next_tile_index
        //        );
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
                if rack.take_tile(ch).is_err() {
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
                rack.add_tile(ch);
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
            self.extend_right(plays, rack, partial_word, edge, target, placement_index + 1);
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
        //        println!(
        //            "Looking for play with word \"{}\" ending at {}",
        //            partial_word,
        //            next_square_index - 1
        //        );
        //        dbg!(&edge);

        if edge.word_terminator {
            let start = next_square_index - partial_word.len();
            let play = self.solving_row.play(start, partial_word.to_string());
            //            println!(" * Found play: {:?} ", play);
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
