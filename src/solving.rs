use crate::scrabble::{
    CheckedRowSquare, Direction, ScrabbleBoard, ScrabblePlay, ScrabbleRack, ScrabbleState,
    BOARD_SIZE,
};
use crate::trie::{Trie, TrieNode};

pub struct SolvingRow {
    direction: Direction,
    index: usize,
    squares: [CheckedRowSquare; BOARD_SIZE],
}

impl SolvingRow {
    pub fn play(&self, end_word_index: usize, word: String) -> ScrabblePlay {
        let start_word_index = end_word_index - word.len();
        let start_position = match self.direction {
            Direction::Vertical => (start_word_index, self.index),
            Direction::Horizontal => (self.index, start_word_index),
        };
        ScrabblePlay {
            start_position,
            direction: self.direction,
            word,
        }
    }
}

pub struct SolvingAnchor<'a> {
    solving_row: &'a SolvingRow,
    anchor_index: usize,
}

impl<'a> SolvingAnchor<'a> {
    pub fn plays(&self, rack: &ScrabbleRack, root: &TrieNode) -> Vec<ScrabblePlay> {
        let mut partial_word = String::with_capacity(BOARD_SIZE);
        let mut plays: Vec<ScrabblePlay> = Vec::new();
        self.add_plays_for_left(
            &mut plays,
            &mut rack.clone(),
            &mut partial_word,
            root,
            self.limit(),
        );
        plays
    }

    fn limit(&self) -> usize {
        for index in (0..self.anchor_index).rev() {
            let square = &self.solving_row.squares[index];
            if square.tile.is_some() || square.is_anchor() {
                return index + 1;
            }
        }
        0
    }

    fn add_plays_for_left(
        &self,
        plays: &mut Vec<ScrabblePlay>,
        rack: &mut ScrabbleRack,
        partial_word: &mut String,
        node: &TrieNode,
        limit: usize,
    ) {
        self.extend_right(plays, rack, partial_word, node, self.anchor_index);
        if limit > 0 {
            for (c, subnode) in node.children.iter() {
                if rack.take_tile(*c).is_ok() {
                    partial_word.push(*c);
                    self.add_plays_for_left(plays, rack, partial_word, subnode, limit - 1);
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
    ) {
        let square = &self.solving_row.squares[index];
        match self.current_play(partial_word, node, index, square) {
            Some(play) => plays.push(play),
            _ => {}
        }
        if index >= BOARD_SIZE {
            return;
        }
        if let Some(c) = square.tile {
            match node.children.get(&c) {
                Some(subnode) => {
                    partial_word.push(c);
                    self.extend_right(plays, rack, partial_word, subnode, index + 1);
                }
                None => {}
            }
        } else {
            for (c, subnode) in node.children.iter() {
                if !rack.take_tile(*c).is_ok() {
                    continue;
                }
                let allowed = match &square.cross_checks {
                    Some(cross_checks) => cross_checks.is_allowed(*c),
                    None => true,
                };
                if !allowed {
                    continue;
                }
                partial_word.push(*c);
                self.extend_right(plays, rack, partial_word, subnode, index + 1);
                rack.add_tile(*c);
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
            if index < BOARD_SIZE {
                return None;
            }
            if let Some(child) = node.children.get(&c) {
                if child.terminal {
                    let play = self.solving_row.play(index - 1, partial_word.to_string());
                    return Some(play);
                }
            }
            None
        } else {
            if node.terminal {
                Some(self.solving_row.play(index - 1, partial_word.to_string()))
            } else {
                None
            }
        }
    }
}

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
                };
                plays.extend(solving_anchor.plays(&self.state.rack, &self.dawg.root()));
            }
        }
        plays
    }

    fn solving_rows(&self) -> Vec<SolvingRow> {
        let board = &self.state.checked_board;
        let mut solving_rows: Vec<SolvingRow> = Vec::with_capacity(2 * BOARD_SIZE);
        for index in 0..BOARD_SIZE {
            for &direction in Direction::iterator() {
                let solving_row = SolvingRow {
                    direction,
                    index,
                    squares: board.row(direction, index),
                };
                solving_rows.push(solving_row);
            }
        }
        solving_rows
    }
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

pub fn solve(vocab_trie: Trie) {
    let rack = ScrabbleRack::new();
    let mut board = ScrabbleBoard::default();
    board.add_word("hello", 7, 7, Direction::Horizontal);
    println!("{}", board.display());
    let checked_board = board.to_checked_board(&vocab_trie);

    for (row_idx, row) in checked_board.squares.iter().enumerate() {
        for (col_idx, square) in row.iter().enumerate() {
            if square.horizontal_cross_checks.is_some() || square.vertical_cross_checks.is_some() {
                println!("{}, {}, {:?}", row_idx, col_idx, square);
            }
        }
    }
    //
    //    let state = ScrabbleState {
    //        checked_board: checked_board,
    //        rack,
    //    };
    //    let solver = Solver {
    //        dawg: vocab_trie,
    //        state,
    //    };
    //    let plays = solver.plays();
    //    println!("{:?}", plays);
}
