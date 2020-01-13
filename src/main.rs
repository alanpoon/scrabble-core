use std::time::Instant;

use crate::loading::{load_vocab, load_vocab_trie};
use crate::solving::solve;

mod loading;
mod scrabble;
mod solving;
mod trie;

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

    let start = Instant::now();
    solve(vocab_trie);
    let duration = start.elapsed();
    println!("Time elapsed in solve() is: {:?}", duration);
}
