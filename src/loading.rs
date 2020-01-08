use std::fs;

use crate::trie::Trie;

pub fn load_vocab() -> Vec<String> {
    let filename = "assets/scrabble_words.txt";
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

pub fn load_vocab_trie(vocab: Vec<String>) -> Trie {
    Trie::new(vocab.iter().map(AsRef::as_ref).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_vocab() {
        assert_eq!(load_vocab().len(), 279_496)
    }

    #[test]
    fn test_load_vocab_trie() {
        let vocab = load_vocab();
        let vocab_trie = load_vocab_trie(vocab);
        assert!(vocab_trie.contains("zymosimeter"));
        assert!(!vocab_trie.contains("zymosometer"))
    }
}
