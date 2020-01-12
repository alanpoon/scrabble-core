use std::collections::HashMap;

//use crate::scrabble::ScrabbleState;

#[derive(Debug, Clone)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub terminal: bool,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            children: HashMap::new(),
            terminal: false,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new(words: Vec<&str>) -> Trie {
        let mut nodes: Vec<TrieNode> = vec![TrieNode {
            children: HashMap::new(),
            terminal: false,
        }];
        for word in words {
            let mut current_node = &mut nodes[0];
            for (i, letter) in word.chars().enumerate() {
                current_node = current_node
                    .children
                    .entry(letter)
                    .or_insert(TrieNode::new());
                if i == word.len() - 1 {
                    current_node.terminal = true;
                }
            }
        }
        Trie { nodes }
    }

    /// Returns the first node  of the Trie unwrapped; this is safe since there will always be a root node
    pub fn root(&self) -> &TrieNode {
        self.nodes.first().unwrap()
    }

    pub fn contains(&self, letters: &str) -> bool {
        let mut node = self.root();
        for ch in letters.chars() {
            match node.children.get(&ch) {
                Some(subnode) => node = subnode,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trie_new() {
        let words = vec!["hello", "world"];
        let trie = Trie::new(words);
        assert!(trie.contains("hello"));
        assert!(!trie.contains("goodbye"));
    }
}
