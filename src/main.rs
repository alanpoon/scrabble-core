mod loading;
mod scrabble;
mod trie;

use crate::loading::{load_vocab, load_vocab_trie};
use crate::scrabble::{Position, ScrabblePlay, ScrabbleState};
use crate::trie::{Trie, TrieNode};
use std::time::Instant;

pub struct Solver {
    dawg: Trie,
    state: ScrabbleState,
}

impl Solver {
    fn plays_for_left_part(
        &mut self,
        partial_word: String,
        node: &TrieNode,
        _limit: usize,
        anchor: Position,
    ) -> Vec<ScrabblePlay> {
        let mut plays = Vec::new();
        plays.extend(self.extend_right(partial_word, node, anchor));
        plays
    }

    fn extend_right(
        &mut self,
        _partial_word: String,
        _node: &TrieNode,
        _square: Position,
    ) -> Vec<ScrabblePlay> {
        vec![]
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
