use std::collections::HashMap;
use std::ops::Index;

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

/// Note: A DawgNode is really just the first DawgEdge in a block associated with a specific node
/// So A DawgNodeIndex is actually a pointer to a DawgEdge
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct DawgNodeIndex(pub usize);

impl DawgNodeIndex {
    fn next(&self) -> DawgNodeIndex {
        DawgNodeIndex(self.0 + 1)
    }
}

pub struct Dawg {
    edges: Vec<DawgEdge>,
}

impl Dawg {
    pub fn contains(&self, word: &str) -> bool {
        let mut target = Some(DawgNodeIndex(0));
        for ch in word.chars() {
            if target.is_none() {
                return false; // No children, but still need to find ch
            }
            let mut node_index = target.unwrap();
            loop {
                let edge = &self[node_index];
                if edge.letter == ch {
                    target = edge.target;
                    break;
                } else if edge.node_terminator {
                    return false;
                }
                node_index = node_index.next();
            }
        }
        true
    }

    pub fn apply_to_children<F>(&self, node: DawgNodeIndex, mut f: F)
    where
        F: FnMut(&DawgEdge),
    {
        let mut node_index = node;

        loop {
            let edge = &self[node_index];
            f(edge);
            if edge.node_terminator {
                break;
            }
            node_index = node_index.next();
        }
    }

    pub fn children(&self, node: DawgNodeIndex) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        self.apply_to_children(node, |edge| (&mut result).push(edge.letter));
        result
    }
}

impl Index<DawgNodeIndex> for Dawg {
    type Output = DawgEdge;

    fn index(&self, index: DawgNodeIndex) -> &Self::Output {
        &self.edges[index.0]
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DawgEdge {
    // 5 bits in paper
    pub letter: char,
    //  1 bit in paper
    pub word_terminator: bool,
    // 1 bit in paper
    pub node_terminator: bool,
    // 16 bits in paper
    pub target: Option<DawgNodeIndex>,
}

impl From<u64> for DawgEdge {
    fn from(input: u64) -> Self {
        const LETTER_OFFSET: u64 = 0;
        const WORD_TERMINATOR_BIT_OFFSET: u64 = 8;
        const NODE_TERMINATOR_BIT_OFFSET: u64 = 16;
        const TARGET_BIT_OFFSET: u64 = 32;

        const MISSING_TARGET_FLAG: usize = 2 ^ 32 - 1;

        let letter = (input >> LETTER_OFFSET) as u8 as char;
        let word_terminator = (input >> WORD_TERMINATOR_BIT_OFFSET) as u8 != 0;
        let node_terminator = (input >> NODE_TERMINATOR_BIT_OFFSET) as u8 != 0;
        let target = match (input >> TARGET_BIT_OFFSET) as usize {
            MISSING_TARGET_FLAG => None,
            other => Some(DawgNodeIndex(other)),
        };

        Self {
            letter,
            word_terminator,
            node_terminator,
            target,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::loading::load_dawg_data;

    #[test]
    fn test_trie_new() {
        let words = vec!["hello", "world"];
        let trie = Trie::new(words);
        assert!(trie.contains("hello"));
        assert!(!trie.contains("goodbye"));
    }

    #[test]
    fn test_load_dawg() {
        let edges = load_dawg_data();
        let dawg = Dawg { edges };
        assert!(dawg.contains("hello"));
        assert!(!dawg.contains("helloworld"));

        let root_children: String = dawg.children(DawgNodeIndex(0)).iter().collect();
        assert_eq!(root_children, "abcdefghijklmnopqrstuvwxyz")
    }

    #[test]
    fn test_load_dawg_2() {}
}
