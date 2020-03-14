use std::ops::{Index, RangeFrom};

/// Note: A DawgNode is really just the first DawgEdge in a block associated with a specific node
/// So A DawgNodeIndex is actually a pointer to a DawgEdge
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct DawgNodeIndex(pub u32);
const NULL_DAWG_NODE_INDEX: u32 = std::u32::MAX;

impl DawgNodeIndex {
    pub fn is_null(&self) -> bool {
        self.0 == NULL_DAWG_NODE_INDEX
    }

    pub fn is_some(&self) -> bool {
        self.0 != NULL_DAWG_NODE_INDEX
    }
}

#[derive(Debug)]
pub struct Dawg {
    pub edges: Vec<DawgEdge>,
}

impl Dawg {
    pub fn root(&self) -> DawgNodeIndex {
        DawgNodeIndex(0)
    }

    pub fn walk_from_node(&self, start: DawgNodeIndex, letters: &str) -> Option<&DawgEdge> {
        let mut maybe_node = start;
        let mut maybe_edge = None;
        for ch in letters.chars() {
            if maybe_node.is_some() {
                maybe_edge = self.leaving_edge(maybe_node, ch);
                if let Some(edge) = maybe_edge {
                    maybe_node = edge.target;
                    continue;
                }
            };
            break;
        }
        maybe_edge
    }

    pub fn walk_from_prior_edge<'a>(
        &'a self,
        prior_edge: &'a DawgEdge,
        letters: &str,
    ) -> Option<&'a DawgEdge> {
        if letters.is_empty() {
            return Some(prior_edge);
        }
        if prior_edge.target.is_some() {
            self.walk_from_node(prior_edge.target, letters)
        } else {
            None
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        self.walk_from_node(self.root(), word).is_some()
    }

    pub fn leaving_edge(&self, node: DawgNodeIndex, ch: char) -> Option<&DawgEdge> {
        for edge in &self[node..] {
            if edge.letter == ch {
                return Some(edge);
            }
            if edge.node_terminator {
                break;
            }
        }
        None
    }

    pub fn apply_to_child_edges<F>(&self, node: DawgNodeIndex, mut f: F)
    where
        F: FnMut(&DawgEdge),
    {
        for edge in self[node..].iter() {
            f(edge);
            if edge.node_terminator {
                break;
            }
        }
    }
}

impl Index<RangeFrom<DawgNodeIndex>> for Dawg {
    type Output = [DawgEdge];

    fn index(&self, index: RangeFrom<DawgNodeIndex>) -> &Self::Output {
        &self.edges[index.start.0 as usize..]
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
    pub target: DawgNodeIndex,
}

pub const DAWG_EDGE_TO_ROOT: &'static DawgEdge = &DawgEdge {
    letter: 'a',
    word_terminator: false,
    node_terminator: false,
    target: DawgNodeIndex(0),
};

impl From<u64> for DawgEdge {
    fn from(input: u64) -> Self {
        const LETTER_OFFSET: u64 = 0;
        const WORD_TERMINATOR_BIT_OFFSET: u64 = 8;
        const NODE_TERMINATOR_BIT_OFFSET: u64 = 16;
        const TARGET_BIT_OFFSET: u64 = 32;

        const MISSING_TARGET_FLAG: u32 = !0; // all ones

        let letter = (input >> LETTER_OFFSET) as u8 as char;
        let word_terminator = (input >> WORD_TERMINATOR_BIT_OFFSET) as u8 != 0;
        let node_terminator = (input >> NODE_TERMINATOR_BIT_OFFSET) as u8 != 0;
        let target = match (input >> TARGET_BIT_OFFSET) as u32 {
            MISSING_TARGET_FLAG => DawgNodeIndex(NULL_DAWG_NODE_INDEX),
            other => DawgNodeIndex(other),
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
    use crate::loading::load_dawg;

    #[test]
    fn test_load_dawg() {
        let dawg = load_dawg();
        assert!(dawg.contains("hello"));
        assert!(!dawg.contains("helloworld"));

        let mut root_children: Vec<char> = Vec::new();
        dawg.apply_to_child_edges(dawg.root(), |edge| (&mut root_children).push(edge.letter));
        let root_children: String = root_children.iter().collect();
        assert_eq!(root_children, "abcdefghijklmnopqrstuvwxyz")
    }
    #[test]
    fn test_load_dawg2() {
        let dawg = load_dawg();
        let k = dawg.walk_from_node(dawg.root(),"pplea");
    }
}
