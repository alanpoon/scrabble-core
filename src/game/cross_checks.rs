use std::fmt;

use crate::dawg::{Dawg, DAWG_EDGE_TO_ROOT};
use crate::loading::A_INDEX;

#[derive(Clone)]
pub struct CrossChecks {
    /// allowed is a bitmask marking which letters are valid for the square
    allowed: u32,
    /// cross_sum should hold the total points associated with neighboring letters for the sake of score computation
    pub cross_sum: i32,
}

impl CrossChecks {
    pub fn set_allowed(&mut self, ch: char) {
        let offset = (ch as u32) - ('a' as u32);
        self.allowed |= 1 << offset;
    }

    pub fn allows(&self, ch: char) -> bool {
        let offset = (ch as u32) - ('a' as u32);
        ((1 << offset) & self.allowed) > 0
    }

    pub fn unwrap_or_empty(string: Option<&String>) -> &str {
        if let Some(string) = string {
            string
        } else {
            ""
        }
    }

    pub fn create(dawg: &Dawg, preceding: &str, following: &str) -> CrossChecks {
        let mut checks = CrossChecks::default();
        let maybe_prior_edge = match preceding.is_empty() {
            true => Some(DAWG_EDGE_TO_ROOT),
            false => dawg.walk_from_node(dawg.root(), preceding),
        };
        if let Some(prior_edge) = maybe_prior_edge {
            let checked_node = prior_edge.target;
            if checked_node.is_some() {
                dawg.apply_to_child_edges(checked_node, |edge| {
                    if let Some(final_edge) = dawg.walk_from_prior_edge(edge, following) {
                        if final_edge.word_terminator {
                            checks.set_allowed(edge.letter);
                        }
                    }
                });
                checks.cross_sum = CrossChecks::cross_sum(preceding, following);
            }
        }
        checks
    }

    fn cross_sum(preceding: &str, following: &str) -> i32 {
        preceding
            .chars()
            .chain(following.chars())
            .map(|ch| 1)
            .sum()
    }

    fn letters(&self) -> String {
        let mut letters = String::new();
        for char_index in 0..27 {
            if self.allowed & (1 << char_index) != 0 {
                letters.push(char::from(A_INDEX + char_index as u8));
            }
        }
        letters
    }

    fn default() -> CrossChecks {
        CrossChecks {
            allowed: 0,
            cross_sum: 0,
        }
    }
}

impl fmt::Debug for CrossChecks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CrossChecks(letters=\"{}\")", self.letters())
    }
}

impl fmt::Display for CrossChecks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CrossChecks(letters=\"{}\")", self.letters())
    }
}
