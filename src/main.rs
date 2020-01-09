mod loading;
mod scrabble;
mod trie;

use crate::loading::{load_vocab, load_vocab_trie};
use crate::scrabble::Direction::{Horizontal, Vertical};
use crate::scrabble::{
    Direction, Position, ScrabbleBoardSquare, ScrabblePlay, ScrabbleRack, ScrabbleState,
    ScrabbleTile,
};
use crate::trie::{Trie, TrieNode};
use std::time::Instant;

pub struct Solver {
    dawg: Trie,
    state: ScrabbleState,
}

impl Solver {
    fn plays(&self) -> Vec<ScrabblePlay> {
        let mut plays: Vec<ScrabblePlay> = Vec::new();
        for solving_row in self.solving_rows() {
            for (anchor_index, tile) in solving_row.squares.iter().enumerate() {
                if !tile.is_anchor() {
                    continue;
                }
                let solving_anchor = SolvingAnchor {
                    solving_row: &solving_row,
                    anchor_index,
                    trie: &self.dawg,
                };
                plays.extend(solving_anchor.plays(&self.state.rack));
            }
        }
        plays
    }

    fn solving_rows(&self) -> Vec<SolvingRow> {
        let board = &self.state.board;
        let rows = board
            .squares
            .iter()
            .enumerate()
            .map(|(row, row_tiles)| SolvingRow {
                direction: Horizontal,
                index: row,
                squares: row_tiles.to_vec(),
            });
        let transposed = board.transposed();
        let cols = transposed
            .squares
            .iter()
            .enumerate()
            .map(|(col, col_tiles)| SolvingRow {
                direction: Vertical,
                index: col,
                squares: col_tiles.to_vec(),
            });
        rows.chain(cols).collect()
    }
}

pub struct SolvingAnchor<'a, 'b> {
    solving_row: &'a SolvingRow,
    anchor_index: usize,
    trie: &'b Trie,
}

impl<'a, 'b> SolvingAnchor<'a, 'b> {
    pub fn plays(&self, rack: &ScrabbleRack) -> Vec<ScrabblePlay> {
        let partial_word = "".to_string();
        let node = self.trie.root();
        self.plays_for_left(&mut rack.clone(), &partial_word, node, self.limit())
    }

    fn limit(&self) -> usize {
        let mut limit = 0;
        for index in (0..self.anchor_index).rev() {
            let square = &self.solving_row.squares[index];
            if square.is_occupied() || square.is_anchor() {
                limit = index + 1;
                break;
            }
        }
        limit
    }

    fn plays_for_left(
        &self,
        rack: &mut ScrabbleRack,
        partial_word: &str,
        node: &TrieNode,
        limit: usize,
    ) -> Vec<ScrabblePlay> {
        let mut plays = self.extend_right(rack, partial_word, node);

        if limit > 0 {
            for (ch, subnode) in node.children.iter() {
                let tile = ScrabbleTile(*ch);
                if rack.take_tile(tile).is_ok() {
                    let mut extended_partial_word = partial_word.to_string();
                    extended_partial_word.push(*ch);
                    let extended_plays =
                        self.plays_for_left(rack, &extended_partial_word, node, limit);
                    plays.extend(extended_plays);
                    rack.replace_tile(tile);
                }
            }
        }
        plays
    }

    fn extend_right(
        &self,
        rack: &mut ScrabbleRack,
        partial_word: &str,
        node: &TrieNode,
    ) -> Vec<ScrabblePlay> {
        vec![]
    }
}

pub struct SolvingRow {
    direction: Direction,
    index: usize,
    squares: Vec<ScrabbleBoardSquare>,
}

//
//struct Dawg {
//    edges: Vec<DawgEdge>
//}
//
//struct DawgEdge {
//    letter: char,
//    next_node_index: usize,
//    terminal: bool,
//    last_edge: bool,
//}
//
//impl Dawg {
//    fn new(words: Vec<String>) -> Dawg {}
//}

fn main() {
    let start = Instant::now();
    let vocab = load_vocab();
    let duration = start.elapsed();
    println!("Time elapsed in load_vocab() is: {:?}", duration);

    let start = Instant::now();
    let vocab_trie = load_vocab_trie(vocab);
    let duration = start.elapsed();
    println!("Time elapsed in load_vocab_trie() is: {:?}", duration);

    let start = Instant::now();
    assert!(vocab_trie.contains("zymosimeter"));
    assert!(!vocab_trie.contains("zymosometer"));
    let duration = start.elapsed();
    println!("Time elapsed in vocab_trie.contains() is: {:?}", duration);
}
