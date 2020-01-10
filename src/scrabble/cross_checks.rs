#[derive(Debug, Clone, Copy)]
pub struct CrossChecks(u32);

impl CrossChecks {
    pub fn allow(&mut self, ch: char) {
        let offset = (ch as u32) - ('a' as u32);
        self.0 |= 1 << offset;
    }

    pub fn is_allowed(&self, ch: char) -> bool {
        let offset = (ch as u32) - ('a' as u32);
        ((1 << offset) & self.0) > 0
    }
}

impl Default for CrossChecks {
    fn default() -> CrossChecks {
        CrossChecks(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cross_checks() {
        let mut checks = CrossChecks::default();
        for c in (b'a'..=b'z').map(char::from) {
            assert!(!checks.is_allowed(c));
            checks.allow(c);
            assert!(checks.is_allowed(c));
        }
    }
}
