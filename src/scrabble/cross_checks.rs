use crate::scrabble::scoring::letter_value;
use crate::trie::Trie;
use std::fmt;

#[derive(Clone, Debug)]
pub struct CrossChecks {
    /// allowed is a bitmask marking which letters are valid for the square
    allowed: u32,
    /// cross_sum should hold the total points associated with neighboring letters for the sake of score computation
    pub cross_sum: u32,
}

impl CrossChecks {
    pub fn allow(&mut self, ch: char) {
        let offset = (ch as u32) - ('a' as u32);
        self.allowed |= 1 << offset;
    }

    pub fn is_allowed(&self, ch: char) -> bool {
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

    pub fn create(trie: &Trie, preceding: &str, following: &str) -> CrossChecks {
        let mut maybe_node = Some(trie.root());
        for ch in preceding.chars() {
            match maybe_node {
                Some(node) => maybe_node = node.children.get(&ch),
                None => break,
            }
        }
        let mut checks = CrossChecks::default();
        if let Some(node) = maybe_node {
            for (ch, subnode) in node.children.iter() {
                let mut maybe_subsubnode = Some(subnode);
                for subch in following.chars() {
                    match maybe_subsubnode {
                        Some(subsubnode) => maybe_subsubnode = subsubnode.children.get(&subch),
                        None => break,
                    }
                }
                if maybe_subsubnode.is_some() {
                    checks.allow(*ch);
                }
            }
            checks.cross_sum = CrossChecks::cross_sum(preceding, following);
        }
        checks
    }

    fn cross_sum(preceding: &str, following: &str) -> u32 {
        preceding
            .chars()
            .chain(following.chars())
            .map(|ch| letter_value(ch))
            .sum()
    }

    fn letters(&self) -> String {
        const A_INDEX: u8 = 97;

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

impl fmt::Display for CrossChecks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CrossChecks(letters=\"{}\")", self.letters())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cross_checks() {
        let mut checks = CrossChecks::default();
        for c in (b'a'..b'z').map(char::from) {
            assert!(!checks.is_allowed(c));
            checks.allow(c);
            assert!(checks.is_allowed(c));
        }
    }

    #[test]
    fn test_display() {
        let mut checks = CrossChecks::default();
        checks.allow('a');
        checks.allow('b');
        checks.allow('z');
        assert_eq!(checks.letters(), "abz");
    }

    #[test]
    fn test_create() {
        let trie = Trie::new(vec!["hello"]);
        assert_eq!(CrossChecks::create(&trie, "he", "lo").letters(), "l");
        assert_eq!(CrossChecks::create(&trie, "he", "oo").letters(), "");
        assert_eq!(CrossChecks::create(&trie, "hl", "lo").letters(), "");
    }

    #[test]
    fn test_create_2() {
        let trie = Trie::new(vec!["yyazz", "yyezz", "yyizz", "yyozz", "yyuzz"]);
        assert_eq!(CrossChecks::create(&trie, "yy", "zz").letters(), "aeiou");
    }
}
