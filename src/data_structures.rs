use std::ops::Index;

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
    pub edges: Vec<DawgEdge>,
}

impl Dawg {
    pub fn root(&self) -> DawgNodeIndex {
        DawgNodeIndex(0)
    }

    pub fn walk_from_node(&self, start: DawgNodeIndex, letters: &str) -> Option<&DawgEdge> {
        let mut maybe_node = Some(start);
        let mut maybe_edge = None;
        for ch in letters.chars() {
            if let Some(node) = maybe_node {
                maybe_edge = self.leaving_edge(node, ch);
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
        prior_edge
            .target
            .and_then(|x| self.walk_from_node(x, letters))
    }

    pub fn contains(&self, word: &str) -> bool {
        self.walk_from_node(self.root(), word).is_some()
    }

    pub fn leaving_edge(&self, node: DawgNodeIndex, ch: char) -> Option<&DawgEdge> {
        let mut node = node;
        loop {
            let edge = &self[node];
            if edge.letter == ch {
                break Some(edge);
            }
            if edge.node_terminator {
                break None;
            }
            node = node.next();
        }
    }

    pub fn apply_to_child_edges<F>(&self, node: DawgNodeIndex, mut f: F)
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

pub const DAWG_EDGE_TO_ROOT: &'static DawgEdge = &DawgEdge {
    letter: 'a',
    word_terminator: false,
    node_terminator: false,
    target: Some(DawgNodeIndex(0)),
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
            MISSING_TARGET_FLAG => None,
            other => Some(DawgNodeIndex(other as usize)),
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

    use super::*;

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
}
