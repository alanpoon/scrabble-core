use std::fs;

use crate::trie::{DawgEdge, Trie};
use std::mem::size_of;

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

pub fn load_dawg_data() -> Vec<DawgEdge> {
    let bytes = fs::read("assets/dawg.bin").expect("Couldn't load asserts/dawg.bin");
    let mut result: Vec<DawgEdge> = Vec::new();
    let u64_size = size_of::<u64>();
    for i in 0..bytes.len() / u64_size {
        let mut num: u64 = 0;
        for j in 0..u64_size {
            let byte_index = u64_size * i + j;
            num += (bytes[byte_index] as u64) << (8 * j) as u64;
        }
        result.push(num.into());
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::trie::DawgNodeIndex;

    //    #[test]
    //    fn test_load_vocab() {
    //        assert_eq!(load_vocab().len(), 279_496)
    //    }
    //
    //    #[test]
    //    fn test_load_vocab_trie() {
    //        let vocab = load_vocab();
    //        let vocab_trie = load_vocab_trie(vocab);
    //        assert!(vocab_trie.contains("zymosimeter"));
    //        assert!(!vocab_trie.contains("zymosometer"))
    //    }
    #[test]
    fn test_load_dawg_data() {
        let dawg_data = load_dawg_data();
        assert_eq!(dawg_data.len(), 190446);
        assert_eq!(
            dawg_data[0],
            DawgEdge {
                letter: 'a',
                word_terminator: false,
                target: Some(DawgNodeIndex(26)),
                node_terminator: false
            }
        );
        assert_eq!(
            dawg_data[1000],
            DawgEdge {
                letter: 'l',
                word_terminator: false,
                target: Some(DawgNodeIndex(136)),
                node_terminator: false
            }
        );
    }
}
